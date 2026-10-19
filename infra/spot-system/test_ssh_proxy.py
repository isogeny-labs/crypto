from pathlib import Path
import tempfile
import unittest
from unittest.mock import Mock,patch

import ssh_proxy
from install_hooks import install


class SshProxyTests(unittest.TestCase):
    def test_verified_aws_key_is_pinned_before_connecting(self):
        with tempfile.TemporaryDirectory() as tmp:
            known=Path(tmp)/'known'
            responses=[{}, {'state':'ready','endpoint_instance_id':'i-current'},
                       {'state':'ready','instance_id':'i-current','host_key':'ssh-ed25519 AAAApublicfixture'}]
            def connect(address,timeout):
                self.assertEqual(known.read_text(),'3.134.144.201 ssh-ed25519 AAAApublicfixture\n')
                self.assertEqual(address,('3.134.144.201',22));return 'connected'
            with patch.object(ssh_proxy,'invoke',side_effect=responses),patch.object(ssh_proxy.socket,'create_connection',side_effect=connect):
                self.assertEqual(ssh_proxy.connect({},'3.134.144.201',22,known),'connected')

    def test_other_hosts_are_never_proxied(self):
        with patch.object(ssh_proxy,'invoke') as invoke:
            with self.assertRaises(ValueError):ssh_proxy.connect({},'other.example',22,Path('/unused'))
            invoke.assert_not_called()

    def test_installer_preserves_ssh_options_and_is_idempotent(self):
        with tempfile.TemporaryDirectory() as tmp:
            home=Path(tmp);(home/'.ssh').mkdir();(home/'.ssh/config').write_text('Host other\n  User alice\n')
            install(home,'example-function');install(home,'example-function')
            config=(home/'.ssh/config').read_text()
            self.assertEqual(config.count('# BEGIN spot-system managed SSH'),1)
            self.assertIn('Host other\n  User alice',config)
            self.assertIn('StrictHostKeyChecking yes',config)
            self.assertTrue((home/'.local/share/spot-system/ssh_proxy.py').is_file())
