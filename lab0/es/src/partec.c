#include <stdio.h>
#include <stdlib.h>
#include <time.h>

typedef struct {
    int type;
    float val;
    long timestamp;
} ValueStruct;

typedef struct {
    int type;
    float val[10];
    long timestamp;
} MValueStruct;

typedef struct {
    int type;
    char message[21];
} MessageStruct;

typedef struct {
    int type;
    union {
        ValueStruct val;
        MValueStruct mvals;
        MessageStruct messages;
    };
} ExportData;

void export(ExportData *data, int n, FILE *pf) {
    fwrite(data, sizeof(ExportData), n, pf);
}

int main() {
    srand(time(NULL));
    ExportData data[100];
    for (int i = 0; i < 100; i++) {
        data[i].type = rand() % 3 + 1;
        switch (data[i].type) {
            case 1:
                data[i].val.type = 1;
                data[i].val.val = (float)(rand() % 100) / 10.0;
                data[i].val.timestamp = time(NULL);
                break;
            case 2:
                data[i].mvals.type = 2;
                for (int j = 0; j < 10; j++) {
                    data[i].mvals.val[j] = (float)(rand() % 100) / 10.0;
                }
                data[i].mvals.timestamp = time(NULL);
                break;
            case 3:
                data[i].messages.type = 3;
                sprintf(data[i].messages.message, "Message %d", i);
                break;
        }
    }
    FILE *pf = fopen("data.bin", "wb");
    if (pf == NULL) {
        printf("Error opening file\n");
        return 1;
    }
    export(data, 100, pf);
    fclose(pf);
    return 0;
}
