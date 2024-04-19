NAME ?=

ifeq ($(NAME),)
	NAME := dummy
endif

INPUT       = $(ROOT)/input
IPT_BUILD   = $(INPUT)/build
IMAGE_REL   = input/build/$(NAME)-test
IMAGE       = $(abspath $(IMAGE_REL))
CC_RV64_PRE = riscv64-linux-gnu-

AS      = $(CC_RV64_PRE)gcc
CC      = $(CC_RV64_PRE)gcc
CXX     = $(CC_RV64_PRE)g++
LD      = $(CC_RV64_PRE)ld
AR      = $(CC_RV64_PRE)ar
OBJDUMP = $(CC_RV64_PRE)objdump
OBJCOPY = $(CC_RV64_PRE)objcopy

SRCS  = tests/$(NAME).c
SRCS += env/start.S \
		env/metal.c

OBJS = $(addprefix $(IPT_BUILD)/, $(addsuffix .o, $(basename $(SRCS))))

LINKER = $(INPUT)/scripts/linker.ld

# Compilation Flags
COMMON_CFLAGS := -fno-pic -march=rv64g -mcmodel=medany -mstrict-align
COMMON_CFLAGS += -march=rv32im_zicsr -mabi=ilp32

CFLAGS  += $(COMMON_CFLAGS) -static
CFLAGS  += -fdata-sections -ffunction-sections
CFLAGS  += -I$(INPUT)/include
CFLAGS  += -DMAINARGS=\"$(mainargs)\"

ASFLAGS += $(COOMMON_CFLAGS) -O0
ASFLAGS += -I$(INPUT)/include

LDFLAGS += -T $(LINKER) \
		   --defsym=_pmem_start=0x80000000 --defsym=_entry_offset=0x0
LDFLAGS += --gc-sections -e _start
LDFLAGS += -melf32lriscv

# Compilation Rules
# Have to add input prefix to '.c' files
# Cause the Makefile work env is in `/remuir`
# Not `/input`
# '.c' -> '.o' : SRCS(.c) -> OBJS
$(IPT_BUILD)/%.o: input/%.c
	@mkdir -p $(dir $@) && echo + CC $<
	$(CC) $(CFLAGS) -c -o $@ $(realpath $<)

# '.S' -> '.o' : SRCS(.S) -> OBJS
$(IPT_BUILD)/%.o: input/%.S
	@mkdir -p $(dir $@) && echo + AS $<
	$(AS) $(ASFLAGS) -c -o $@ $(realpath $<)

# '.o' -> 'IMAGE.elf': OBJS -> ELF
# If there occurs circular dependency for '_start'
# Use --start-group --end-group pair options
# Not simply using $^
$(IMAGE).elf: $(OBJS)
	@echo + LD "->" $(IMAGE_REL).elf
	$(LD) $(LDFLAGS) -o $(IMAGE).elf $^

# Tags
$(IMAGE).bin: $(IMAGE).elf
	@$(OBJDUMP) -d $(IMAGE).elf > $(IMAGE).txt
	@echo + OBJCOPY "->" $(IMAGE_REL).bin
	@$(OBJCOPY) -S --set-section-flags .bss=alloc,contents -O binary $(IMAGE).elf $(IMAGE).bin

image: $(IMAGE).bin
