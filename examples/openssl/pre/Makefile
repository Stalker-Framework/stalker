EXPS_x86_64 = ev0.1 ev0.3.1 ev0.3.1-decrypt ev0.3.2 ev0.3.3
EXPS_riscv_64 = original

EXPS = ${EXPS_${PLATFORM}_${ARCH}}

OLIBS_x86_64 = libcrypto.so.1.1
OLIBS_riscv_64 =
OLIBS = ${OLIBS_${PLATFORM}_${ARCH}}

CC = gcc

# path
SRC_PATH = src
LIB_PATH = lib
BUILD_PATH = build/obj
BIN_PATH = build/bin

# executable
BIN_NAME = run

# run exps
RUN_x86_64 = $(BIN_PATH)/$(BIN_NAME)
RUN = ${RUN_${PLATFORM}_${ARCH}}

# extensions
SRC_EXT = c

# code lists
SOURCES = $(shell find $(SRC_PATH) -name '*.$(SRC_EXT)' | sort -k 1nr | cut -f2-)
OBJECTS = $(SOURCES:$(SRC_PATH)/%.$(SRC_EXT)=$(BUILD_PATH)/%.o)
DEPS = $(OBJECTS:.o=.d)

# flags
COMPILE_FLAGS = -Wall -Wextra -g -w
INCLUDES = -I include/
LIBS = -lcrypto

.PHONY: default_target
default_target: release

.PHONY: release
release: export CFLAGS := $(CFLAGS) $(COMPILE_FLAGS)
release: dirs
	@$(MAKE) all

.PHONY: dirs
dirs:
	@echo "Creating directories"
	@mkdir -p $(dir $(OBJECTS))
	@mkdir -p $(BIN_PATH)

.PHONY: clean
clean:
	@echo "Deleting directories"
	@$(RM) -r $(BUILD_PATH)
	@$(RM) -r $(BIN_PATH)

# checks the executable and symlinks to the output
.PHONY: all
all: $(BIN_PATH)/$(BIN_NAME)

# Creation of the executable
$(BIN_PATH)/$(BIN_NAME): $(OBJECTS)
	@echo "Linking: $@"
	$(CC) $(OBJECTS) -o $@ $(LIBS)

# Add dependency files, if they exist
-include $(DEPS)

# Source file rules
$(BUILD_PATH)/%.o: $(SRC_PATH)/%.$(SRC_EXT)
	@echo "Compiling: $< -> $@"
	$(CC) $(CCFLAGS) $(INCLUDES) -MP -MMD -c $< -o $@

.PHONY: run
run: release
	@echo "Running:"
	./run -p $(PLATFORM) -a $(ARCH) -r "$(RUN)" -e "$(EXPS)" -l "$(OLIBS)"