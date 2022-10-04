build: ./src/endianess.c
	clang ./src/endianess.c -o ./endianess

run: build
	./endianess

clean: ./endianess
	@rm ./endianess

all: build run clean
