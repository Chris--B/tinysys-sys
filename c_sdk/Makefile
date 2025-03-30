# Glob to find all the source files in the SDK.
# Some notes:
#     1. This should handle nested directories without issue
#     2. This will ignore any *.cpp files, and will need to be updated if those show up.
SDK_HDRS:=$(shell find SDK -type f -name "*.h" | sort)
SDK_SRCS:=$(shell find SDK -type f -name "*.c" | sort)
# Track different object files for debug or release builds
SDK_OBJS_DBG:=$(SDK_SRCS:SDK/%.c=_build/dbg/%.o)
SDK_OBJS_REL:=$(SDK_SRCS:SDK/%.c=_build/rel/%.o)

# These are important and lifted from the Makefiles in the tinysys project
RISCV_SYSROOT:=$(shell riscv64-unknown-elf-gcc -print-sysroot)
CXXFLAGS_BASE:=\
	-std=c++20                          \
	-I "SDK/"                           \
	--target=riscv32                    \
	-march=rv32im_zicsr_zifencei_zfinx  \
	-mabi=ilp32                         \
	--sysroot=$(RISCV_SYSROOT)          \
	-D_LIBCPP_HAS_NO_THREADS

ifeq ($(OS),Windows_NT)
    # Not sure what to do here. I recommend building as Linux from WSL.
else
    UNAME_S := $(shell uname -s)
    ifeq ($(UNAME_S),Linux)
        # Use the default C++ compiler
    endif
    ifeq ($(UNAME_S),Darwin)
        # Note: We compile everything using LLVM from homebrew to make sure that
        # bindgen agrees on what `uint32_t` is defined as: gcc says ulong, clang/bindgen says uint.
        CXX:=$(shell brew --prefix llvm)/bin/clang++
    endif
endif

.PHONY: all build clean update info info2 

all: build

update:
	./update_sdk.sh

# TODO: Add and use libtinysys_sdk_debug.a
build: libtinysys_sdk.a objdump_libtinysys_sdk.txt

# Note: This will clean the "local" copy of sdk.rs.
# But `make build` modifies the one in src/sdk.rs too. That is not touched by `make clean`.
clean:
	@rm -vrf _build 
	@rm -vf libtinysys_sdk.a libtinysys_sdk_debug.a
	@rm -vf sdk.rs
	@rm -vf objdump_libtinysys_sdk.txt

libtinysys_sdk_debug.a: $(SDK_OBJS_DBG)
	riscv64-unknown-elf-ar -rc $@ $^

libtinysys_sdk.a: $(SDK_OBJS_REL)
	riscv64-unknown-elf-ar -rc $@ $^

_build/dbg/%.o: SDK/%.c
	@mkdir -p _build/dbg/
	$(CXX) -x c++ -g -O0 -c $^ -o $@ $(CXXFLAGS_BASE)

_build/rel/%.o: SDK/%.c
	@mkdir -p _build/rel/
	$(CXX) -x c++ -g -Os -c $^ -o $@ $(CXXFLAGS_BASE)

objdump_%.txt: %.a
	riscv64-unknown-elf-objdump -tC $< > $@

# TODO: Move this out of build.rs into a dedicated script
sdk.rs: $(SDK_HDRS) ../build.rs
	BINDGEN_EXTRA_CLANG_ARGS="--sysroot=$(RISCV_SYSROOT)" cargo build --features=bindgen $*
	cargo fmt
	git diff -- ../src/sdk.rs | diffstat -Cm || true
	@cp -v ../src/sdk.rs .

# Useful to see what the script thinks is going on while using it
info:
	riscv64-unknown-elf-gcc --version
	@echo
	$(CXX) --version

# Useful to see what the script thinks is going on while working on it
info2:
	@echo SDK_HDRS=$(SDK_HDRS)
	@echo SDK_SRCS=$(SDK_SRCS)
	@echo SDK_OBJS_DBG=$(SDK_OBJS_DBG)
	@echo SDK_OBJS_REL=$(SDK_OBJS_REL)
