#include <netdb.h>      // addrinfo
#include <netinet/in.h> // sockaddr_in
#include <sys/socket.h> // sockaddr
#include <arpa/inet.h>

#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <errno.h>
#include <limits.h>

#define BUFSIZE 512
#define MIN_PARAMS 3
#define MAX_PARAMS 4

extern int errno;

void fatal_system(const char*);
void fatal(const char*);
int is_number(char*);
uint16_t to_port(const char*);
void tcp(const char*, const uint16_t);

int main(int argc, char ** argv) {
  uint16_t port;
  if (argc < MIN_PARAMS || argc > MAX_PARAMS) {
    fatal("error parsing args. Use <host> <port> [-u]");
  }

  if (!is_number(argv[2])) {
    fatal("second argument is not a number");
  } else if ((port = to_port(argv[2])) == 0) {
    fatal("second argument is not a number between 1 and 65535");
  }

  tcp(argv[1], port);
  
  return 0;
}

void tcp(const char* host, const uint16_t port) {
  struct sockaddr_in sa;

  memset(&sa, 0, sizeof(sa));
  sa.sin_family = AF_INET;
  sa.sin_port = htons(port);

  if (inet_pton(sa.sin_family, host, &(sa.sin_addr)) < 0) {
    fatal_system("inet_pton");
  }

  int fd;
  if ((fd = socket(sa.sin_family, SOCK_STREAM, 0)) < 0) {
    fatal_system("socket");
  }

  if (connect(fd, (const struct sockaddr*) &sa, sizeof(sa)) != 0) {
    fatal_system("connect");
  }

  char* input = (char *) malloc(BUFSIZE * sizeof(char));
  for (;;) {
    int bytes_send,
        bytes_recv;
    fgets(input, BUFSIZE, stdin);
    if ((bytes_send = send(fd, input, strlen(input), 0)) <= 0) {
      printf("error bytes_send\n");
      continue;
    }

    memset(input, 0, bytes_send);
    if ((bytes_recv = recv(fd, input, BUFSIZE, 0)) <= 0) {
      printf("error bytes_recv\n");
      continue;
    }
    printf("%s", input);

    memset(input, 0, bytes_recv);
  }
}

void fatal_system(const char* msg) {
  char* err = (char*) malloc(50 * sizeof(char));
  strerror_r(errno, err, 50);
  fprintf(stderr, "%s with error %s\n", msg, err);

  free(err);

  exit(1);
}

void fatal(const char* msg) {
  fprintf(stderr, "%s\n", msg);
  exit(1);
}

int is_number(char *input) {
  char* tmp = input;
  while (*tmp != '\0' && *tmp >= 0x30 && *tmp <= 0x39) tmp++;
  return *tmp == '\0';
}

uint16_t to_port(const char *input) {
  int port = atoi(input);
  return port > USHRT_MAX || port < 0 ? 0 : (uint16_t) port;
}

/**
* struct addrinfo {
*   int               ai_flags;
*   int               ai_family;
*   int               ai_socktype;
*   int               ai_protocol;
*   socklen_t         ai_addrlen;
*   struct sockaddr  *ai_addr;
*   char             *ai_canonname;
*   struct addrinfo  *ai_next;
* };
*
* struct sockaddr {
*   unsigned short  sa_family;
*   char            sa_data[14];
* };
*
* typedef unsigned short int sa_family_t;
* typedef uint16_t in_port_t;
* typedef uint32_t in_addr_t;
*
* \//sin = Socket address INternet.
* struct sockaddr_in {
*   sa_family_t     sin_family;                       \// 2 bytes
*   in_port_t       sin_port;                         \// 2 bytes
*   struct in_addr  sin_addr;                         \// 4 bytes
*   unsigned char   sin_zero[sizeof (struct sockaddr) \// (2 + 14) -2 -2 -4 = 8bytes
*       - __SOCKADDR_COMMON_SIZE
*       - sizeof (in_port_t)
*       - sizeof (struct in_addr)];
* };
*
* struct addr_in {
*   in_addr_t s_addr;
* };
*/
