CC := cargo

# Rust modules
MODELS := models
DA := data_access
BL := business_logic
DI := di_container
LOGGER := logger
SNAPS_GEN := snaps_gen

# Rust executable files
API := api
SNAP_INS := snap_insertions
SNAP_INS_MES := snap_insert_mes

# Colors
BOLD := \033[1m
BOLD_GREEN := \033[1;32m
RESET := \033[0m

# Directories
TARGET_DIR := ./build
DEBUG_DIR := $(TARGET_DIR)/debug
RELEASE_DIR := $(TARGET_DIR)/release

EXE = $(DEBUG_DIR)/api

CUR_DIR := 

# Compile keys
RELEASE_KEY := --release

.PHONY: all clean debug build release run

all: debug release

build: debug

debug: $(DEBUG_DIR)
	@echo "=$(BOLD_GREEN)COMPILING$(RESET)=============================================================================="

	@cd ./src && \
	$(CC) build -p $(MODELS) && \
	$(CC) build -p $(DA) && \
	$(CC) build -p $(BL) && \
	$(CC) build -p $(DI) && \
	$(CC) build -p $(LOGGER) && \
	$(CC) build -p $(SNAPS_GEN) && \
	$(CC) build -p $(API) && \
	$(CC) build -p $(SNAP_INS) && \
	$(CC) build -p $(SNAP_INS_MES) && \
	cp ./target/$@/api ../$^/ && \
	cp ./target/$@/snap_insertions ../$^/ && \
	cp ./target/$@/snap_insert_mes ../$^/ && \
	cp ./target/$@/*.rlib ../$^/
	
	@$(MAKE) log-info CUR_DIR=$^

release: $(RELEASE_DIR)
	@echo "=$(BOLD_GREEN)COMPILING$(RESET)=============================================================================="

	@cd ./src && \
	$(CC) build $(RELEASE_KEY) -p $(MODELS) && \
	$(CC) build $(RELEASE_KEY) -p $(DA) && \
	$(CC) build $(RELEASE_KEY) -p $(BL) && \
	$(CC) build $(RELEASE_KEY) -p $(DI) && \
	$(CC) build $(RELEASE_KEY) -p $(LOGGER) && \
	$(CC) build $(RELEASE_KEY) -p $(SNAPS_GEN) && \
	$(CC) build $(RELEASE_KEY) -p $(API) && \
	$(CC) build $(RELEASE_KEY) -p $(SNAP_INS) && \
	$(CC) build $(RELEASE_KEY) -p $(SNAP_INS_MES) && \
	cp ./target/$@/api ../$^/ && \
	cp ./target/$@/snap_insertions ../$^/ && \
	cp ./target/$@/snap_insert_mes ../$^/ && \
	cp ./target/$@/*.rlib ../$^/ 
	
	@$(MAKE) log-info CUR_DIR=$^

BUILD := debug
LOG_LEVEL := info
run:
	@$(MAKE) $(BUILD)
	RUST_LOG=$(LOG_LEVEL) ./build/$(BUILD)/api

$(RELEASE_DIR):
	mkdir -p $(TARGET_DIR)
	mkdir -p $(RELEASE_DIR)

$(DEBUG_DIR):
	mkdir -p $(TARGET_DIR)
	mkdir -p $(DEBUG_DIR)

snap-inserts-run:
	@$(MAKE) $(BUILD)
	RUST_LOG=$(LOG_LEVEL) ./build/$(BUILD)/snap_insertions

snap-inserts-mes-run:
	@$(MAKE) $(BUILD)
	RUST_LOG=$(LOG_LEVEL) ./build/$(BUILD)/snap_insert_mes


log-info:
	@echo "=$(BOLD_GREEN)INFO$(RESET)==================================================================================="
	@echo "$(BOLD)Исполнямые файлы:$(RESET)"
	@echo "  • ./$(CUR_DIR)/api"
	@echo "  • ./$(CUR_DIR)/snap_insertions"
	@echo "$(BOLD)Запуск:$(RESET)"
	@echo "  • API: [RUST_LOG=<уровень>] ./$(CUR_DIR)/api"
	@echo "  • Snap insertions: [RUST_LOG=<уровень>] ./$(CUR_DIR)/snap_insertions"
	@echo ""
	@echo "$(BOLD)Доступные уровни логирования:$(RESET)"
	@echo "  • RUST_LOG=error   - только критические ошибки"
	@echo "  • RUST_LOG=warn    - ошибки и предупреждения"
	@echo "  • RUST_LOG=info    - основная информация о работе (по умолчанию)"
	@echo "  • RUST_LOG=debug   - технические детали для разработчиков"
	@echo "  • RUST_LOG=trace   - максимальная детализация"
	@echo ""
	@echo "$(BOLD)Альтереативный запуск:$(RESET)"
	@echo "make <snap-inserts-<mes->>run [BUILD=<release|debug>] [LOG_LEVEL=<уровень>]"
	@echo ""
	@echo "$(BOLD)По умолчанию:$(RESET)"
	@echo "  • BUILD=debug"
	@echo "  • LOG_LEVEL=info"
	@echo =========================================================================================


clean:
	rm -rf $(TARGET_DIR) && \
	cd ./src && cargo clean