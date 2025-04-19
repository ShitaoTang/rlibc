# Top-level Makefile for rlibc
include config.mak

CARGO ?= $(shell which cargo)
CARGO_TARGET_DIR ?= target

PREFIX      ?= $(prefix)
INCLUDEDIR  ?= $(includedir)
LIBDIR      ?= $(PREFIX)/lib
BINDIR      ?= $(PREFIX)/bin

SRCDIR := .
GENH = obj/include/bits/alltypes.h obj/include/bits/syscall.h
C_DIR = $(SRCDIR)/C
C_LIBDIR = $(C_DIR)/lib
C_TESTDIR = $(C_DIR)/test
TOOL_LIBS = lib/rlibc-gcc.specs

ifeq ($(TARGET),)
    CARGO_TARGET_DIR := target
    TARGET_FLAG :=
    TARGET_DIR :=
else
    TARGET_FLAG := --target=$(TARGET)
    TARGET_DIR := $(TARGET)/
endif

ifeq ($(DEBUG),1)
    PROFILE := debug
    BUILD_FLAG := 
else
    PROFILE := release
    BUILD_FLAG := --release
endif

RUST_LIB := $(CARGO_TARGET_DIR)/$(TARGET_DIR)$(PROFILE)/librlibc.a
C_LIB := $(C_LIBDIR)/librlibc_helper.a

.PHONY: all
all: build $(TOOL_LIBS)

lib/rlibc-gcc.specs: $(SRCDIR)/tools/rlibc-gcc.specs.sh config.mak
	sh $< "$(INCLUDEDIR)" "$(LIBDIR)" > $@

obj/include/bits/alltypes.h: \
    $(SRCDIR)/arch/$(ARCH)/bits/alltypes.h.in \
    $(SRCDIR)/include/alltypes.h.in \
    $(SRCDIR)/tools/mkalltypes.sed
	@mkdir -p obj/include/bits
	@echo "📄 Generating $@"
	@sed -f $(SRCDIR)/tools/mkalltypes.sed \
		$(SRCDIR)/arch/$(ARCH)/bits/alltypes.h.in \
		$(SRCDIR)/include/alltypes.h.in > $@

obj/include/bits/syscall.h: $(SRCDIR)/arch/$(ARCH)/bits/syscall.h.in
	@echo "📄 Generating $@"
	@mkdir -p obj/include/bits
	@cp $< $@
	@sed -n -e 's/__NR_/SYS_/p' < $< >> $@

$(C_LIB): $(GENH)
	@echo "📦 Building C library in $(C_DIR)..."
	$(MAKE) -C $(C_DIR) all ARCH=$(ARCH)

.PHONY: build
build: $(GENH) $(C_LIB)
	@echo "📦 Building rlibc (debug=$(DEBUG), target=$(TARGET))..."
	$(CARGO) build $(BUILD_FLAG) $(TARGET_FLAG)

ARCH_INCLUDE := arch/$(ARCH)/

.PHONY: install
install:
	@echo "🚚 Installing rlibc to $(PREFIX)..."

	install -d $(DESTDIR)$(LIBDIR)
	install -m644 $(RUST_LIB) $(DESTDIR)$(LIBDIR)/librlibc.a
	install -m644 $(C_LIB) $(DESTDIR)$(LIBDIR)/librlibc_helper.a

	install -m644 $(TOOL_LIBS) $(DESTDIR)$(LIBDIR)/

	install -d $(DESTDIR)$(INCLUDEDIR)
	cp -r include/* $(DESTDIR)$(INCLUDEDIR)/
	cp -r $(ARCH_INCLUDE)/* $(DESTDIR)$(INCLUDEDIR)/

	install -d $(DESTDIR)$(INCLUDEDIR)/bits
	install -m644 obj/include/bits/*.h $(DESTDIR)$(INCLUDEDIR)/bits/

.PHONY: clean
clean:
	@echo "🧹 Cleaning Rust build artifacts..."
	$(CARGO) clean
	$(MAKE) -C $(C_DIR) clean

.PHONY: distclean
distclean: clean
	@echo "🧹 Removing generated config files..."
	rm -f config.mak
