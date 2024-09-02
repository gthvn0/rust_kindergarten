#define NBDKIT_API_VERSION 1

#include <nbdkit-plugin.h>
#include <stdlib.h>
#include <string.h>

#define THREAD_MODEL NBDKIT_THREAD_MODEL_SERIALIZE_ALL_REQUESTS

#define SIZE 512 // Bytes

static int memory_config(const char *key, const char *value) { return -1; }

static void *memory_open(int readonly) {
  void *handle = malloc(SIZE);

  if (!handle) {
    nbdkit_error("Failed to allocate memory");
    return NULL;
  }

  memset(handle, 0, SIZE);
  return handle;
}

static void memory_close(void *handle) { free(handle); }

static int64_t memory_get_size(void *handle) { return SIZE; }

static int memory_pread(void *handle, void *buf, uint32_t count,
                        uint64_t offset) {
  // Currently always return hello
  memcpy(buf, "hello", 5);
  return 5;
}

// Only ".name", ".open", ".get_size" and ".pread"
static struct nbdkit_plugin plugin = {
    .name = "memory",
    .open = memory_open,
    .get_size = memory_get_size,
    .pread = memory_pread,
    .config = memory_config,
    .close = memory_close,
};

NBDKIT_REGISTER_PLUGIN(plugin)
