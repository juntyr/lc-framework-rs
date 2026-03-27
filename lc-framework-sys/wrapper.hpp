#include <iostream>

#include "lc.h"

extern "C" int
lc_compress(const size_t npepros,
    const LC_CPUpreprocessor* const prepro_ids,
    const size_t* const prepros_nparams,
    const double* const prepros_params,
    const size_t ncomps,
    const LC_CPUcomponents* const comp_ids,
    const byte* const input,
    const long long insize,
    byte** encoded,
    long long* encsize)
{
    byte* hpreencdata = nullptr;
    byte* hencoded = nullptr;

    try {
        if (ncomps > sizeof(long long)) {
            fprintf(stderr, "ERROR: too many components\n\n");
            throw std::runtime_error("LC error");
        }

        if (insize <= 0) {
            fprintf(stderr, "ERROR: input too small\n\n");
            throw std::runtime_error("LC error");
        }
        if (insize >= std::numeric_limits<long long>::max()) {
            fprintf(stderr, "ERROR: input too large\n\n");
            throw std::runtime_error("LC error");
        }

        // extract preprocessor list and their parameters
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

        // create component chain for encoding
        unsigned long long comp_chain = 0;
        for (auto i = 0; i < ncomps; i++) {
            comp_chain |= ((byte)comp_ids[i]) << (i * 8);
        }

        // allocate CPU memory for preprocessed data
        hpreencdata = new byte[insize];
        std::copy(input, input + insize, hpreencdata);

        // encode with CPU preprocessors
        long long hpreencsize = insize;
        h_preprocess_encode(hpreencsize, hpreencdata, prepros);

        // allocate CPU memory for encoded data
        const long long hchunks = (hpreencsize + CS - 1) / CS; // round up
        const long long hmaxsize = 2 * sizeof(long long) + hchunks * sizeof(short) + hchunks * CS; // MB: adjust later
        hencoded = new byte[hmaxsize];

        // encode with CPU component chain
        long long hencsize = 0;
        h_encode(comp_chain, hpreencdata, hpreencsize, hencoded, hencsize);

        // clean up and return encoded data
        delete[] hpreencdata;

        *encoded = hencoded;
        *encsize = hencsize;

        return 0;
    } catch (const std::exception& ex) {
        std::cerr << ex.what() << std::endl;

        // clean up, if necessary
        if (hpreencdata != nullptr) {
            delete[] hpreencdata;
        }
        if (hencoded != nullptr) {
            delete[] hencoded;
        }

        return -1;
    }
}

extern "C" void
lc_free_bytes(byte* data)
{
    delete[] data;
}

extern "C" int
lc_decompress(const size_t npepros,
    const LC_CPUpreprocessor* const prepro_ids,
    const size_t* const prepros_nparams,
    const double* const prepros_params,
    const size_t ncomps,
    const LC_CPUcomponents* const comp_ids,
    const byte* const encoded,
    const long long encsize,
    byte** decoded,
    long long* decsize)
{
    byte* hencoded = nullptr;
    byte* hdecoded = nullptr;

    try {
        if (ncomps > sizeof(long long)) {
            fprintf(stderr, "ERROR: too many components\n\n");
            throw std::runtime_error("LC error");
        }

        // extract preprocessor list and their parameters
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

        // create component chain for encoding
        unsigned long long comp_chain = 0;
        for (auto i = 0; i < ncomps; i++) {
            comp_chain |= ((byte)comp_ids[i]) << (i * 8);
        }

        // create component chain for decoding
        unsigned long long comp_schain = comp_chain;
        if (comp_schain != 0) {
            while ((comp_schain >> 56) == 0) {
                comp_schain <<= 8;
            }
        }

        // read the size of the preprocessed data
        long long pre_size = 0;
        assert(encsize >= sizeof(pre_size));
        std::memcpy(&pre_size, encoded, sizeof(pre_size));

        // allocate CPU memory for decoded data
        hencoded = new byte[std::max(pre_size, encsize)];
        std::copy(encoded, encoded + encsize, hencoded);
        hdecoded = new byte[pre_size];

        // decode with CPU component chain
        long long hdecsize = 0;
        h_decode(comp_schain, hencoded, hdecoded, hdecsize);

        // reuse CPU memory of decoded data to postprocess
        assert(hdecsize <= pre_size);
        byte* hpredecdata = hdecoded;

        // decode with CPU preprocessors
        long long hpredecsize = hdecsize;
        h_preprocess_decode(hpredecsize, hpredecdata, prepros);

        // clean up and return decoded data
        delete[] hencoded;
        // hdecoded points to the same memory as hpredecdata

        *decoded = hpredecdata;
        *decsize = hpredecsize;

        return 0;
    } catch (const std::exception& ex) {
        std::cerr << ex.what() << std::endl;

        // clean up, if necessary
        if (hencoded != nullptr) {
            delete[] hencoded;
        }
        if (hdecoded != nullptr) {
            delete[] hdecoded;
        }

        return -1;
    }
}
