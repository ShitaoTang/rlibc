RLIBCC = rlibc-gcc
MUSLGCC = musl-gcc

CFLAGS = -g -static

SRCS := $(wildcard test_*.c)
BASENAMES := $(basename $(SRCS))

RLIBC_BINS := $(addsuffix _rlibc, $(BASENAMES))
MUSL_BINS := $(addsuffix _musl, $(BASENAMES))

all: $(RLIBC_BINS) $(MUSL_BINS)

rlibc: $(RLIBC_BINS)

musl: $(MUSL_BINS)

%_rlibc: %.c
	$(RLIBCC) $< -o $@ $(CFLAGS)

%_musl: %.c
	$(MUSLGCC) $< -o $@ $(CFLAGS)

$(BASENAMES):
	$(MAKE) $@_rlibc
	$(MAKE) $@_musl

run_%: %
	@echo "🔧 Running ./$@_rlibc..."
	@./$*_rlibc > $*_rlibc.out || echo "❌ $*_rlibc crashed!"
	@echo "🔧 Running ./$@_musl..."
	@./$*_musl  > $*_musl.out  || echo "❌ $*_musl crashed!"
	@echo "🔍 Comparing output..."
	@diff -u $*_rlibc.out $*_musl.out && echo "✅ Output matches!" || echo "❌ Output differs!"

list:
	@echo "📝 Available test run targets:"
	@for f in $(SRCS); do \
		name=$$(basename $$f .c); \
		echo "  make run_$$name"; \
	done

clean:
	rm -f *_rlibc *_musl *.out *.log
