CC = gcc
BIN_NAME = bin
CRYPTOLIB_DIST =

# flags
COMPILE_FLAGS = -Wall -Wextra -g -w
LIB_PATH = ${CRYPTOLIB_DIST}/${LIB}

ifeq (${LIB},mbedtls)
	LIB_LINK = mbedcrypto
else ifeq (${LIB},libgcrypt)
	LIB_LINK = gcrypt
else ifeq (${LIB},openssl)
	LIB_LINK = crypto
else ifeq (${LIB},libressl)
	LIB_LINK = crypto
endif

DEPS = -I../../utils -I$(LIB_PATH)/include -L$(LIB_PATH)/lib -l$(LIB_LINK)

.PHONY: default_target
default_target: all

.PHONY: clean
clean:
	@echo "Deleting builds."
	@$(RM) -r $(BIN_NAME) core

# checks the executable and symlinks to the output
.PHONY: all
all: $(BIN_NAME)

# Creation of the executable
$(BIN_NAME):
	@echo "Building: $@"
	$(CC) $(COMPILE_FLAGS) $(DEPS) -o $@ ../../utils/print.c main.c
