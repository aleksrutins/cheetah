LIBS += -lcmark
.PHONY: all

all: hyena

hyena: main.o
	$(CC) $(CFLAGS) $(LIBS) -o hyena main.o
