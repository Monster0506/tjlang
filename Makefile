.PHONY: all clean build run

VCPKG_ROOT = C:/install/vcpkg
TOOLCHAIN = $(VCPKG_ROOT)/scripts/buildsystems/vcpkg.cmake
TRIPLET = x64-windows
GEN_DIR = gen/grammar
BUILD_DIR = build
CONFIG = Release
EXE = $(BUILD_DIR)/$(CONFIG)/tjlang.exe

all: build

clean:
	-rmdir /s /q $(BUILD_DIR)

gen:
	powershell -ExecutionPolicy Bypass -File scripts/gen.ps1

configure: gen
	cmake -B $(BUILD_DIR) -S . -DCMAKE_TOOLCHAIN_FILE=$(TOOLCHAIN) -DVCPKG_TARGET_TRIPLET=$(TRIPLET) -DANTLR_GEN_DIR=$(GEN_DIR)

build: configure
	cmake --build $(BUILD_DIR) --config $(CONFIG)
	

run: build
	set PATH=$(VCPKG_ROOT)/installed/$(TRIPLET)/bin;$(PATH)
	$(EXE)
