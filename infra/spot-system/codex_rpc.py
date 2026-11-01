"""Local-only Codex JSON-RPC over its Unix WebSocket control socket.

No approval requests are answered by this client. Callers must leave them for
the configured approval workflow or the user.
"""
import base64
import hashlib
import json
import os
import socket
import struct
import time


class Client:
    def __init__(self, path, timeout=30):
        self.socket = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
        self.socket.settimeout(timeout)
        self.socket.connect(str(path))
        self.timeout = timeout
        self.sequence = 0
        self.buffer = b''
        key = base64.b64encode(os.urandom(16)).decode()
        self.socket.sendall(('GET / HTTP/1.1\r\nHost: localhost\r\nUpgrade: websocket\r\n'
                             'Connection: Upgrade\r\nSec-WebSocket-Version: 13\r\n'
                             'Sec-WebSocket-Key: '+key+'\r\n\r\n').encode())
        while b'\r\n\r\n' not in self.buffer:
            chunk = self.socket.recv(4096)
            if not chunk or len(self.buffer) > 16384:
                self.close()
                raise ValueError('Invalid local WebSocket handshake')
            self.buffer += chunk
        headers, self.buffer = self.buffer.split(b'\r\n\r\n', 1)
        lines = headers.decode('ascii').split('\r\n')
        fields = {name.lower(): value.strip() for name, value in
                  (line.split(':', 1) for line in lines[1:] if ':' in line)}
        expected = base64.b64encode(hashlib.sha1((key+'258EAFA5-E914-47DA-95CA-C5AB0DC85B11').encode()).digest()).decode()
        if lines[0].split()[1] != '101' or fields.get('sec-websocket-accept') != expected:
            self.close()
            raise ValueError('Local WebSocket upgrade was rejected')

    def close(self):
        self.socket.close()

    def read(self, size):
        while len(self.buffer) < size:
            data = self.socket.recv(min(65536, size-len(self.buffer)))
            if not data:
                raise EOFError('Codex control socket closed')
            self.buffer += data
        result, self.buffer = self.buffer[:size], self.buffer[size:]
        return result

    def send_frame(self, payload, opcode=1):
        length = len(payload)
        prefix = bytes([0x80 | opcode])
        if length < 126:
            prefix += bytes([0x80 | length])
        elif length <= 65535:
            prefix += b'\xfe' + struct.pack('!H', length)
        else:
            prefix += b'\xff' + struct.pack('!Q', length)
        mask = os.urandom(4)
        self.socket.sendall(prefix + mask + bytes(b ^ mask[i % 4] for i, b in enumerate(payload)))

    def send(self, message):
        self.send_frame(json.dumps(message).encode())

    def receive(self):
        fragments = bytearray()
        while True:
            first, second = self.read(2)
            opcode, length = first & 15, second & 127
            if first & 0x70 or second & 0x80:
                raise ValueError('Unexpected WebSocket extensions or server masking')
            if length == 126:
                length = struct.unpack('!H', self.read(2))[0]
            elif length == 127:
                length = struct.unpack('!Q', self.read(8))[0]
            if length + len(fragments) > 64 * 1024 * 1024:
                raise ValueError('Oversized Codex RPC message')
            payload = self.read(length)
            if opcode == 8:
                raise EOFError('Codex closed its WebSocket')
            if opcode == 9:
                self.send_frame(payload, 10)
                continue
            if opcode == 10:
                continue
            if opcode not in (0, 1):
                raise ValueError('Unexpected Codex WebSocket message')
            fragments.extend(payload)
            if first & 0x80:
                return json.loads(fragments)

    def request(self, method, params=None):
        self.sequence += 1
        sequence = self.sequence
        self.send({'id': sequence, 'method': method, 'params': params or {}})
        deadline = time.monotonic() + self.timeout
        while time.monotonic() < deadline:
            self.socket.settimeout(max(.1, deadline-time.monotonic()))
            message = self.receive()
            if message.get('id') == sequence and 'method' not in message:
                if 'error' in message:
                    raise RuntimeError('Codex RPC '+method+' failed: '+str(message['error']))
                return message['result']
        raise TimeoutError(method)

    def initialize(self):
        result = self.request('initialize', {'clientInfo': {'name': 'spot_recovery', 'version': '1.0'},
                                            'capabilities': {'experimentalApi': True}})
        self.send({'method': 'initialized'})
        return result

    def inventory(self):
        result = []
        for thread_id in self.request('thread/loaded/list').get('data', []):
            thread = self.request('thread/read', {'threadId': thread_id, 'includeTurns': False})['thread']
            result.append({key: thread.get(key) for key in ('id', 'cwd', 'source', 'status')})
        return result
