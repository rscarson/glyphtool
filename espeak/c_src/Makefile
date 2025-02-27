CC = clang
WARNINGS = -Wno-attributes -Wno-deprecated-declarations -Wno-pointer-sign -Wno-int-conversion
CFLAGS = -Iinclude -fvisibility=hidden -fno-exceptions -fwrapv $(WARNINGS)

# Set platform specific flags
ifdef OS
	RM = del /Q
	LIB_EXT = .lib
	EXEC_EXT = .exe
	FixPath = $(subst /,\,$1)
else
	RM = rm -f
	LIB_EXT = .a
	EXEC_EXT =
	FixPath = $1
endif

# Define targets
LIB_OUT = espeak_mini$(LIB_EXT)
OBJ_DIR = obj
LIB_DIR = lib

# UCD sources
_UCD = ucd/case.o ucd/categories.o ucd/ctype.o ucd/proplist.o ucd/scripts.o ucd/tostring.o
UCD = $(patsubst %,$(OBJ_DIR)/%,$(_UCD))

# Espeak sources
_OBJ = common.o mnemonics.o error.o ieee80.o compiledata.o compiledict.o dictionary.o encoding.o intonation.o langopts.o numbers.o phoneme.o phonemelist.o readclause.o setlengths.o soundicon.o spect.o ssml.o synthdata.o synthesize.o tr_languages.o translate.o translateword.o voices.o wavegen.o speech.o espeak_api.o
OBJ = $(patsubst %,$(OBJ_DIR)/%,$(_OBJ))

# Obj compilation rule
$(OBJ_DIR)/%.o: src/%.c
	$(CC) -c -o $@ $< $(CFLAGS)

# Clean up rule
clean: 
	$(RM) $(call FixPath, $(OBJ_DIR)/*.o $(OBJ_DIR)/ucd/*.o $(OBJ_DIR)/*.d $(OBJ_DIR)/ucd/*.d $(LIB_DIR)/espeak_mini.* example/example$(EXEC_EXT))

# Compiles the static library
all: $(LIB_DIR)/$(LIB_OUT) example/example$(EXEC_EXT)
$(LIB_DIR)/$(LIB_OUT): $(OBJ) $(UCD)
	ar rcs $@ $^

# Build the example project
example/example$(EXEC_EXT): example/example.c $(LIB_DIR)/$(LIB_OUT)
	$(CC) -o $@ $< -Iinclude -L$(LIB_DIR) -lespeak_mini