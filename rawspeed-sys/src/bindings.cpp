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

void* rawspeed_rawimage_decode(const uint8_t* data, size_t size, const void* metadata_ptr, int scale, char** error_msg) {
  *error_msg = nullptr;
  try {
    auto metadata = (const CameraMetaData*) metadata_ptr;
    Buffer buffer(data, size);
    RawParser parser(&buffer);
    auto decoder = parser.getDecoder();
    decoder->failOnUnknown = true;
    decoder->applyCrop = true;
    if (scale) {
      decoder->interpolateBadPixels = true;
    }
    decoder->checkSupport(metadata);
    decoder->decodeRaw();
    decoder->decodeMetaData(metadata);
    if (scale) {
      decoder->mRaw->scaleBlackWhite();
    }
    auto raw = decoder->mRaw;
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

RawspeedImageInfo rawspeed_rawimage_info(const void* ptr) {
  auto image = (RawImage*)ptr;
  RawspeedImageInfo info;
  info.data = (*image)->getDataUncropped(0, 0);
  info.width = (*image)->getUncroppedDim().x;
  info.height = (*image)->getUncroppedDim().y;
  info.pitch = (*image)->pitch;
  info.cropped_data = (*image)->getData();
  info.cropped_width = (*image)->dim.x;
  info.cropped_height = (*image)->dim.y;
  return info;
}
