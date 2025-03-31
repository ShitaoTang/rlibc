CC = gcc
CFLAGS = -Wall -Wextra -nostdlib -fno-builtin -fno-stack-protector -no-pie -fPIC -ggdb -O1
AR = ar
RANLIB = ranlib

SRC_DIR = src
INCLUDE_DIR = include
OBJ_DIR = obj
LIB_DIR = lib
TEST_DIR = test

SRCS = $(wildcard $(SRC_DIR)/*.c)
OBJS = $(patsubst $(SRC_DIR)/%.c, $(OBJ_DIR)/%.o, $(SRCS))
LIB = $(LIB_DIR)/librlibc_helper.a
TEST_SRCS = $(wildcard $(TEST_DIR)/*.c)
TEST_OBJS = $(patsubst $(TEST_DIR)/%.c, $(OBJ_DIR)/%.o, $(TEST_SRCS))
TEST_BINS = $(patsubst $(TEST_DIR)/%.c, $(TEST_DIR)/%, $(TEST_SRCS))

ARCH ?= $(shell uname -m)
ifeq ($(ARCH), x86_64)
	CFLAGS += -D__x86_64__
else ifeq ($(ARCH), aarch64)
	CFLAGS += -D__aarch64__
else
	$(error) "Unsupported architecture: $(ARCH)"
endif

.PHONY: all clean dirs

all: dirs $(LIB) test run

dirs:
	@mkdir -p $(LIB_DIR) $(OBJ_DIR) $(TEST_DIR)

$(OBJ_DIR)/%.o: $(SRC_DIR)/%.c
	$(CC) $(CFLAGS) -I$(INCLUDE_DIR) -c $< -o $@

$(LIB): $(OBJS)
	$(AR) rcs $@ $^
	$(RANLIB) $@

$(OBJ_DIR)/%.o: $(TEST_DIR)/%.c
	$(CC) $(CFLAGS) -I$(INCLUDE_DIR) -c $< -o $@

$(TEST_DIR)/%: $(OBJ_DIR)/%.o $(LIB)
	$(CC) $(CFLAGS) -o $@ $< -L$(LIB_DIR) -lrlibc_helper

test: $(TEST_BINS)

run:
	@rm $(TEST_DIR)/test
	@make test
	@$(TEST_DIR)/test

clean:
	rm -rf $(LIB_DIR) $(OBJ_DIR) 
