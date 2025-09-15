.PHONY: all clean gen configure build run json

# vcpkg + CMake settings
VCPKG_ROOT = C:/install/vcpkg
TOOLCHAIN  = $(VCPKG_ROOT)/scripts/buildsystems/vcpkg.cmake
TRIPLET    = x64-windows
GEN_DIR    = gen/grammar
BUILD_DIR  = build
NINJA_DIR  = build-ninja
CONFIG    ?= Release
EXE        = $(BUILD_DIR)/$(CONFIG)/tjlang.exe

# Default generator = Visual Studio
GEN ?= VS

# Generator-specific flags and build command
ifeq ($(GEN),Ninja)
  GEN_FLAGS  = -G Ninja \
               -DCMAKE_BUILD_TYPE=$(CONFIG) \
               -DCMAKE_EXPORT_COMPILE_COMMANDS=ON
  BUILD_CMD  = cmake --build $(BUILD_DIR) --parallel
else ifeq ($(GEN),VS)
  GEN_FLAGS  = -G "Visual Studio 17 2022"
  BUILD_CMD  = cmake --build $(BUILD_DIR) --config $(CONFIG) --parallel
else
  $(error Unknown GEN '$(GEN)'. Use GEN=Ninja or GEN=VS)
endif

all: build

clean:
	-rmdir /s /q $(BUILD_DIR)
	-rmdir /s /q $(NINJA_DIR)

gen:
	powershell -ExecutionPolicy Bypass -File scripts/gen.ps1

# Configure with selected generator (default = VS, full build)
configure: gen
	cmake -B $(BUILD_DIR) -S . \
		$(GEN_FLAGS) \
		-DCMAKE_TOOLCHAIN_FILE=$(TOOLCHAIN) \
		-DVCPKG_TARGET_TRIPLET=$(TRIPLET) \
		-DANTLR_GEN_DIR=$(GEN_DIR)

# Build target (Visual Studio full build)
build: configure
	$(BUILD_CMD)

# Run the executable (adds vcpkg bin to PATH first)
run: build
	set PATH=$(VCPKG_ROOT)/installed/$(TRIPLET)/bin;$(PATH) && $(EXE)

# Generate compile_commands.json (uses Ninja in its own dir, no build required)
json: 
	cmake -B $(NINJA_DIR) -S . \
		-G Ninja \
		-DCMAKE_BUILD_TYPE=$(CONFIG) \
		-DCMAKE_EXPORT_COMPILE_COMMANDS=ON \
		-DCMAKE_TOOLCHAIN_FILE=$(TOOLCHAIN) \
		-DVCPKG_TARGET_TRIPLET=$(TRIPLET) \
		-DANTLR_GEN_DIR=$(GEN_DIR)
	copy "$(NINJA_DIR)\compile_commands.json" .
	@echo "compile_commands.json ready at project root."
