#include <iostream>

#include "lc.h"

extern "C" int lc_compress(
    const size_t npepros,
    const LC_CPUpreprocessor* const prepro_ids,
    const size_t* const prepros_nparams,
    const double* const prepros_params,
    const size_t ncomps,
    const LC_CPUcomponents* const comp_ids,
    const byte* const input,
    const long long insize,
    byte** encoded,
    long long* encsize
) {
    byte* hpreencdata = nullptr;
    byte* hencoded = nullptr;

    try {
        std::vector<std::pair<byte, std::vector<double>>> prepros;
        size_t prepros_params_cnt = 0;
        for (auto i = 0; i < npepros; ++i) {
            std::vector<double> params = {
                prepros_params + prepros_params_cnt,
                prepros_params + prepros_params_cnt + prepros_nparams[i],
            };
            prepros_params_cnt += prepros_nparams[i];
            prepros.push_back(std::make_pair((byte)prepro_ids[i], params));
        }

        std::vector<std::vector<byte>> comp_list;
        for (auto i = 0; i < ncomps; ++i) {
            std::vector<byte> comps = { (byte)comp_ids[i] };
            comp_list.push_back(comps);
        }
        auto stages = comp_list.size();

        if (insize <= 0) {
            fprintf(stderr, "ERROR: input too small\n\n"); throw std::runtime_error("LC error");
        }
        if (insize >= std::numeric_limits<long long>::max()) {
            fprintf(stderr, "ERROR: input too large\n\n"); throw std::runtime_error("LC error");
        }

        // CPU preprocessor encoding
        hpreencdata = new byte [insize];
        std::copy(input, input + insize, hpreencdata);
        long long hpreencsize = insize;
        h_preprocess_encode(hpreencsize, hpreencdata, prepros);

        // allocate CPU memory
        const long long hchunks = (hpreencsize + CS - 1) / CS;  // round up
        const long long hmaxsize = 2 * sizeof(long long) + hchunks * sizeof(short) + hchunks * CS;  //MB: adjust later
        hencoded = new byte [hmaxsize];
        long long hencsize = 0;

        // create chain for current combination and output
        unsigned long long combin = 0;
        unsigned long long chain = 0;
        for (auto s = 0; s < stages; s++) {
            unsigned long long compnum = comp_list[s][(combin >> (s * 8)) & 0xff];
            chain |= compnum << (s * 8);
        }

        // CPU encoding
        h_encode(chain, hpreencdata, hpreencsize, hencoded, hencsize);

        delete [] hpreencdata;

        *encoded = hencoded;
        *encsize = hencsize;

        return 0;
    }
    catch(const std::exception &ex) {
        std::cerr << ex.what() << std::endl;

        if (hpreencdata != nullptr) delete [] hpreencdata;
        if (hencoded != nullptr) delete [] hencoded;

        return -1;
    }
}

extern "C" void lc_free_bytes(byte* data) {
    delete [] data;
}

extern "C" int lc_decompress(
    const size_t npepros,
    const LC_CPUpreprocessor* const prepro_ids,
    const size_t* const prepros_nparams,
    const double* const prepros_params,
    const size_t ncomps,
    const LC_CPUcomponents* const comp_ids,
    const byte* const encoded,
    const long long encsize,
    byte** decoded,
    long long* decsize
) {
    byte* hencoded = nullptr;
    byte* hdecoded = nullptr;
    byte* hpredecdata = nullptr;

    try {
        std::vector<std::pair<byte, std::vector<double>>> prepros;
        size_t prepros_params_cnt = 0;
        for (auto i = 0; i < npepros; ++i) {
            std::vector<double> params = {
                prepros_params + prepros_params_cnt,
                prepros_params + prepros_params_cnt + prepros_nparams[i],
            };
            prepros_params_cnt += prepros_nparams[i];
            prepros.push_back(std::make_pair((byte)prepro_ids[i], params));
        }

        std::vector<std::vector<byte>> comp_list;
        for (auto i = 0; i < ncomps; ++i) {
            std::vector<byte> comps = { (byte)comp_ids[i] };
            comp_list.push_back(comps);
        }
        auto stages = comp_list.size();

        long long pre_size = 0;
        assert(encsize >= sizeof(pre_size));
        std::memcpy(&pre_size, encoded, sizeof(pre_size));

        // allocate CPU memory
        hencoded = new byte [std::max(pre_size, encsize)];
        std::copy(encoded, encoded + encsize, hencoded);
        hdecoded = new byte [pre_size];
        long long hdecsize = 0;

        // create chain for current combination and output
        unsigned long long combin = 0;
        unsigned long long chain = 0;
        for (auto s = 0; s < stages; s++) {
            unsigned long long compnum = comp_list[s][(combin >> (s * 8)) & 0xff];
            chain |= compnum << (s * 8);
        }

        // CPU decoding
        unsigned long long schain = chain;
        if (chain != 0) {
          while ((schain >> 56) == 0) schain <<= 8;
        }
        h_decode(schain, encoded, hdecoded, hdecsize);

        // CPU preprocessor decoding
        hpredecdata = new byte [hdecsize];
        std::copy(hdecoded, hdecoded + hdecsize, hpredecdata);
        long long hpredecsize = hdecsize;
        h_preprocess_decode(hpredecsize, hpredecdata, prepros);

        delete [] hencoded;
        delete [] hdecoded;

        *decoded = hpredecdata;
        *decsize = hpredecsize;

        return 0;
    }
    catch(const std::exception &ex) {
        std::cerr << ex.what() << std::endl;

        if (hencoded != nullptr) delete [] hdecoded;
        if (hdecoded != nullptr) delete [] hdecoded;
        if (hpredecdata != nullptr) delete [] hpredecdata;

        return -1;
    }
}
