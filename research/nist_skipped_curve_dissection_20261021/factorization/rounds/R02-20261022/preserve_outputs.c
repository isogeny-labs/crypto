// Retain native outputs across tool cleanup. No arithmetic or log polling.
#define _GNU_SOURCE
#include <dirent.h>
#include <errno.h>
#include <fcntl.h>
#include <limits.h>
#include <poll.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/inotify.h>
#include <sys/stat.h>
#include <sys/syscall.h>
#include <unistd.h>

static int source_fd, retained_fd;

static void retain(const char *name) {
    struct stat st;
    char saved[NAME_MAX + 1];
    if (fstatat(source_fd, name, &st, AT_SYMLINK_NOFOLLOW) != 0) return;
    if (!S_ISREG(st.st_mode)) return;
    int n = snprintf(saved, sizeof(saved), "%s.inode-%llu", name,
                     (unsigned long long)st.st_ino);
    if (n < 0 || (size_t)n >= sizeof(saved)) {
        fprintf(stderr, "name too long: %s\n", name);
        exit(1);
    }
    if (linkat(source_fd, name, retained_fd, saved, 0) == 0) {
        printf("%s\t%s\n", name, saved);
    } else if (errno != EEXIST && errno != ENOENT) {
        perror("linkat");
        exit(1);
    }
}

static void scan(void) {
    int copy = dup(source_fd);
    if (copy < 0) { perror("dup"); exit(1); }
    DIR *dir = fdopendir(copy);
    if (!dir) { perror("fdopendir"); exit(1); }
    rewinddir(dir);
    struct dirent *entry;
    while ((entry = readdir(dir)) != NULL) retain(entry->d_name);
    closedir(dir);
}

int main(int argc, char **argv) {
    if (argc < 4) {
        fprintf(stderr, "usage: preserve-outputs SOURCE RETAINED OWNER_PID...\n");
        return 2;
    }
    if (mkdir(argv[2], 0700) != 0 && errno != EEXIST) { perror("mkdir"); return 1; }
    source_fd = open(argv[1], O_RDONLY | O_DIRECTORY | O_CLOEXEC);
    retained_fd = open(argv[2], O_RDONLY | O_DIRECTORY | O_CLOEXEC);
    int notify = inotify_init1(IN_CLOEXEC | IN_NONBLOCK);
    if (source_fd < 0 || retained_fd < 0 || notify < 0) { perror("open/inotify"); return 1; }
    if (inotify_add_watch(notify, argv[1], IN_CREATE | IN_MOVED_TO | IN_CLOSE_WRITE) < 0) {
        perror("inotify_add_watch"); return 1;
    }
    setvbuf(stdout, NULL, _IOLBF, 0);
    scan();
    struct pollfd *fds = calloc((size_t)argc - 2, sizeof(*fds));
    if (!fds) { perror("calloc"); return 1; }
    fds[0] = (struct pollfd){notify, POLLIN, 0};
    int active = 0;
    for (int i = 3; i < argc; i++) {
        char *end;
        long owner = strtol(argv[i], &end, 10);
        if (!*argv[i] || *end || owner <= 0 || owner > INT_MAX) return 2;
        int process = (int)syscall(SYS_pidfd_open, (pid_t)owner, 0);
        if (process < 0 && errno != ESRCH) { perror("pidfd_open"); return 1; }
        fds[i - 2] = (struct pollfd){process, POLLIN, 0};
        if (process >= 0) active++;
    }
    if (!active) return 0;
    union { struct inotify_event align; char bytes[65536]; } buffer;
    for (;;) {
        if (poll(fds, (nfds_t)argc - 2, -1) < 0) {
            if (errno == EINTR) continue;
            perror("poll"); return 1;
        }
        ssize_t count = read(notify, buffer.bytes, sizeof(buffer.bytes));
        if (count < 0 && errno != EAGAIN && errno != EINTR) { perror("read"); return 1; }
        for (ssize_t offset = 0; offset < count;) {
            struct inotify_event *event = (struct inotify_event *)(buffer.bytes + offset);
            if (event->mask & IN_Q_OVERFLOW) { fprintf(stderr, "inotify overflow\n"); return 1; }
            if (event->len) retain(event->name);
            offset += (ssize_t)sizeof(*event) + event->len;
        }
        for (int i = 1; i < argc - 2; i++) {
            if (fds[i].fd >= 0 && fds[i].revents) {
                close(fds[i].fd);
                fds[i].fd = -1;
                active--;
            }
        }
        if (!active) { scan(); break; }
    }
    free(fds);
    close(notify);
    close(source_fd);
    close(retained_fd);
    return 0;
}
