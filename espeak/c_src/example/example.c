#include <stdlib.h>
#include <string.h>

#include <espeak_ng.h>

int readfile(const char* filename, char** buffer, long* size) {
    FILE* file;
    fopen_s(&file, filename, "rb");
    if (!file) return -1;

    fseek(file, 0, SEEK_END);
    *size = ftell(file);

    *buffer = (char*)malloc(*size + 1);
    if (!*buffer) {
        fclose(file);
        return -1;
    }

    fseek(file, 0, SEEK_SET);
    fread(*buffer, 1, *size, file);

    fclose(file);
    return 0;
}

int main(int argc, char* argv[]) {
    PHONEME_CONFIGS data;

    readfile("data/phontab", &data.tab, &data.data_len);
    readfile("data/phonindex", &data.index, &data.index_len);
    readfile("data/phondata", &data.data, &data.data_len);
    readfile("data/intonations", &data.intonations, &data.intonation_len);

    char * voice;
    long voice_size;
    readfile("data/en-US", &voice, &voice_size);

    char * dict;
    long dict_size;
    readfile("data/en_dict", &dict, &dict_size);

    espeak_Initialize(
        AUDIO_OUTPUT_RETRIEVAL, 0, &data, espeakINITIALIZE_DONT_EXIT | espeakINITIALIZE_PHONEME_IPA
    );


    espeak_ERROR  result2 = espeak_SetVoiceByBuffer("en-us", voice, voice_size, dict, dict_size);
    
    char text[255] = "neighbourhood";

    char* input = text;



    const char* result = espeak_TextToPhonemes((const void **)&input, espeakCHARS_UTF8, espeakINITIALIZE_PHONEME_IPA);
    if (result == NULL) {
        printf("Error!");
    } else {
        printf("Phonemes: %s\n", result);
    }
}