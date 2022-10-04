#include <netdb.h>
#include <bits/types.h> // uint8_t, uint16_t, uint32_t, uint64_t

#include <stdio.h>
#include <stdlib.h>

#include <limits.h>

void binary(long);

int main() {
  uint16_t port_host_endian = 80;
  uint16_t port_network_endian = htons(port_host_endian);

  binary(port_host_endian);     // little-endian
  binary(port_network_endian);  // big-endian

  return 0;
}

void binary(long port) {
  if (port < 0 || port > USHRT_MAX) {
    return;
  }
  char* b = (char *) malloc(16 * sizeof(char));

  int count = 0;
  while (port > 1) {
    b[count++] = port % 2 + 0x30;
    port /= 2;
  }
  b[count++] = port % 2 + 0x30;

  for (int i = count; i < 16; i++) {
    b[i] = 0x30;
  }

  for (int i = 15; i >= 0; i--) {
    if (i == 7) {
      printf(" ");
    }
    printf("%c", b[i]);
  }
  printf("\n");

  free(b);
}

