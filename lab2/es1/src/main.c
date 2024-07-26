#include <stdio.h>
#include <stdlib.h>

#define N 100


typedef struct
{
    int type;
    float val;
    long timestamp;
} ValueStruct;


typedef struct
{
    int type;
    float val[10];
    long timestamp;
} MValueStruct;


typedef struct
{
    int type;
    char message[21];
} MessageStruct;


typedef struct
{
    int type;
    union
    {
        ValueStruct val;
        MValueStruct mvals;
        MessageStruct messages;
    };
    
} ExportData;


void export(ExportData *data, int n, FILE *fp);



void export(ExportData *data, int n, FILE *fp)
{
    fwrite(data, sizeof(ExportData), n, fp);
}


int main()
{
    FILE *fp = fopen("es1.txt", "w");

    if (fp == NULL)
        exit(EXIT_FAILURE);
    
    ExportData data[N]; 
    export(data, N, fp);
    printf("Lughezza: %d", (int) sizeof(ExportData));

    return 0;    
}