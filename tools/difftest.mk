DIFF_REF_PATH = tools/spike-diff
DIFF_REF_SO = $(DIFF_REF_PATH)/build/spike-diff-so

ARGS_DIFF = --diff=$(DIFF_REF_SO)

$(DIFF_REF_SO):
	$(MAKE) -s -C $(DIFF_REF_PATH)

.PHONY: $(DIFF_REF_SO)
