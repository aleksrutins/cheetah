.PHONY: all

all: hyena

hyena: main.o
	gcc -o hyena main.o -lcmark
