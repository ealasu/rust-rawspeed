#include "RawSpeed-API.h"
#include "metadata/CameraMetadataException.h"

using namespace std;
using namespace rawspeed;

CameraMetaData* rawspeed_metadata_init(const char* filename) {
  try {
    return new CameraMetaData(filename);
  } catch (CameraMetadataException &e) {
    // Reading metadata failed. e.what() will contain error message.
    // TODO: include error details
    return nullptr;
  } catch (...) {
    return nullptr;
  }
}

void rawspeed_metadata_delete(CameraMetaData* ptr) {
  delete ptr;
}

RawImage* rawspeed_rawimage_decode(const uchar8* data, size_t size, const CameraMetaData* metadata) {
  try {
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
  } catch (...) {
    return nullptr;
  }
}

void rawspeed_rawimage_delete(RawImage* ptr) {
  delete ptr;
}

uchar8* rawspeed_rawimage_data(RawImage* ptr) {
  return (*ptr)->getData();
}

int rawspeed_rawimage_width(RawImage* ptr) {
  return (*ptr)->dim.x;
}

int rawspeed_rawimage_height(RawImage* ptr) {
  return (*ptr)->dim.y;
}

int rawspeed_rawimage_pitch(RawImage* ptr) {
  return (*ptr)->pitch;
}
