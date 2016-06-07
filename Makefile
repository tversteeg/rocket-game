NAME=game

RM=rm -rf
CFLAGS=-g -Wall -pedantic -O3 -DUSE_GLEW
LDLIBS=-fopenmp -lccore -lGL -lGLU -lGLEW -lm -lX11 -lXrandr -lpthread

SRCS=src/game.c src/render.c src/player.c
OBJS=$(subst .c,.o,$(SRCS))

all: $(NAME)

$(NAME): $(OBJS)
	$(CC) $(LDFLAGS) -o $(NAME) $(OBJS) $(LDLIBS)

.PHONY: clean
clean:
	$(RM) $(OBJS) $(NAME)
