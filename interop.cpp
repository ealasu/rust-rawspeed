#include "RawSpeed-API.h"

namespace rawspeed {
  std::unique_ptr<const Buffer> new_buffer(const uchar8* data, uint32 size) {

    //Buffer buf(data, size);

    //unique_ptr<uchar8, decltype(&alignedFree)> buf = 
    std::unique_ptr<uchar8, decltype(&alignedFree)> buf(data, &alignedFree);
    return std::make_unique<Buffer>(std::move(buf), size);
  }

  void do_stuff() {
  }
}
