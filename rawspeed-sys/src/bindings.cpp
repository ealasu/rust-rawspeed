#include "RawSpeed-API.h"
#include "metadata/CameraMetadataException.h"
#include "bindings.h"

using namespace std;
using namespace rawspeed;

void* rawspeed_metadata_init(const char* filename, char** error_msg) {
  *error_msg = nullptr;
  try {
    return new CameraMetaData(filename);
  } catch (exception &e) {
    auto w = new string(e.what());
    *error_msg = w->data();
    return nullptr;
  } catch (...) {
    return nullptr;
  }
}

void rawspeed_metadata_free(void* ptr) {
  delete (CameraMetaData*)ptr;
}

void* rawspeed_rawimage_decode(const uint8_t* data, size_t size, const void* metadata_ptr, char** error_msg) {
  *error_msg = nullptr;
  try {
    auto metadata = (const CameraMetaData*) metadata_ptr;
    Buffer buffer(data, size);
    RawParser parser(&buffer);
    auto decoder = parser.getDecoder();
    decoder->failOnUnknown = true;
    decoder->checkSupport(metadata);
    decoder->decodeRaw();
    decoder->decodeMetaData(metadata);
    auto raw = decoder->mRaw;
    raw->scaleBlackWhite();
    return new RawImage(raw);
  } catch (exception &e) {
    auto w = new string(e.what());
    *error_msg = w->data();
    return nullptr;
  } catch (...) {
    return nullptr;
  }
}

void rawspeed_rawimage_free(void* ptr) {
  delete (RawImage*)ptr;
}

uint8_t* rawspeed_rawimage_data(const void* ptr) {
  return (*(RawImage*)ptr)->getData();
}

int rawspeed_rawimage_width(const void* ptr) {
  return (*(RawImage*)ptr)->dim.x;
}

int rawspeed_rawimage_height(const void* ptr) {
  return (*(RawImage*)ptr)->dim.y;
}

int rawspeed_rawimage_pitch(const void* ptr) {
  return (*(RawImage*)ptr)->pitch;
}
