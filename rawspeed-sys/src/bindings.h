#include "stdint.h"
#include "stdlib.h"

typedef struct {
  uint8_t* data;
  int width;
  int height;
  int pitch;
  uint8_t* cropped_data;
  int cropped_width;
  int cropped_height;
} RawspeedImageInfo;

#ifdef __cplusplus
extern "C" {
#endif

void* rawspeed_metadata_init(const char* filename, char** error_msg);
void rawspeed_metadata_free(void* ptr);
void* rawspeed_rawimage_decode(const uint8_t* data, size_t size, const void* metadata_ptr, char** error_msg);
void rawspeed_rawimage_free(void* ptr);
RawspeedImageInfo rawspeed_rawimage_info(const void* ptr);
//uint8_t* rawspeed_rawimage_data(const void* ptr);
//int rawspeed_rawimage_width(const void* ptr);
//int rawspeed_rawimage_height(const void* ptr);
//int rawspeed_rawimage_pitch(const void* ptr);

#ifdef __cplusplus
}
#endif
