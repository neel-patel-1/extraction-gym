FEATURES ?=
FLAGS=--release --features=$(FEATURES)

EXTRACTORS=$(shell cargo run -q $(FLAGS) -- --extractor=print)

PROGRAM=target/release/extraction-gym

SRC=$(shell find . -name '.rs') Cargo.toml Cargo.lock
DATA=$(shell find data/lut-synth -name '*.json')

TARGETS=

.PHONY: all
all: bench

define run-extraction
$(info Running extractor: $(2) on data: $(1))
TARGETS += $(1:data/%=output/%)-$(2).json
$(1:data/%=output/%)-$(2).json: $(1)
	@mkdir -p $$(dir $$@)
	$(PROGRAM) $$< --extractor=$(2) --out=$$@
endef

$(foreach ext,$(EXTRACTORS),\
	$(foreach data,$(DATA),\
        $(eval $(call run-extraction,$(data),$(ext)))\
    )\
)

.PHONY: bench
bench: plot.py $(TARGETS)
	./$^

$(PROGRAM): $(SRC)
	cargo build $(FLAGS)

.PHONY: test
test:
	cargo test --release

.PHONY: nits
nits:
	rustup component add rustfmt clippy
	cargo fmt -- --check
	cargo clean --doc

	cargo clippy --tests