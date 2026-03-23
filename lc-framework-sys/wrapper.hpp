#include <cassert>
#include <iostream>
#include <sstream>

#include "lc.h"

static std::string available_preprocessors_impl() {
    std::stringstream ss;

    const std::map<std::string, byte> prepro_name2num = getPreproMap();
    for (auto pair: prepro_name2num) {
        ss << pair.first << " ";
    }

    return ss.str();
}

extern "C" const char* lc_available_preprocessors() {
    static std::string preprocessors = available_preprocessors_impl();
    return preprocessors.c_str();
}

static std::string available_components_impl() {
    std::stringstream ss;

    const std::map<std::string, byte> comp_name2num = getCompMap();
    for (auto pair: comp_name2num) {
        ss << pair.first << " ";
    }

    return ss.str();
}

extern "C" const char* lc_available_components() {
    static std::string components = available_components_impl();
    return components.c_str();
}

extern "C" int lc_compress(
    const char* const prepros_cstr,
    const char* const comp_cstr,
    const byte* const input,
    const long long insize,
    byte** encoded,
    long long* encsize
) {
    byte* hpreencdata = nullptr;
    byte* hencoded = nullptr;

    try {
        // generate preprocessor maps
        std::map<std::string, byte> prepro_name2num = getPreproMap();
        std::string prepro_num2name [256];
        for (auto pair: prepro_name2num) {
        prepro_num2name[pair.second] = pair.first;
        }

        // generate component maps
        std::map<std::string, byte> comp_name2num = getCompMap();
        std::string comp_num2name [256];
        for (auto pair: comp_name2num) {
        comp_num2name[pair.second] = pair.first;
        }

        int stages;
        unsigned long long algorithms;
        std::vector<std::pair<byte, std::vector<double>>> prepros;
        std::vector<std::vector<byte>> comp_list;

        auto prepros_str = std::string(prepros_cstr);
        auto comp_str = std::string(comp_cstr);

        prepros = getItems(prepro_name2num, prepros_str.data());
        comp_list = getStages(comp_name2num, comp_str.data(), stages, algorithms);
        if (algorithms != 1) {
            fprintf(stderr, "ERROR: pipeline must describe one algorithm\n\n");
            throw std::runtime_error("LC error");
        }

        printStages(prepros, prepro_name2num, comp_list, comp_name2num, stages, algorithms);

        if (insize <= 0) {fprintf(stderr, "ERROR: input too small\n\n"); throw std::runtime_error("LC error");}
        if (insize >= 9223372036854775807) {fprintf(stderr, "ERROR: input too large\n\n"); throw std::runtime_error("LC error");}
        printf("input size: %lld bytes\n\n", insize);

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
        for (int s = 0; s < stages; s++) {
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
    const char* const prepros_cstr,
    const char* const comp_cstr,
    const byte* const encoded,
    const long long encsize,
    byte** decoded,
    long long* decsize
) {
    byte* hdecoded = nullptr;
    byte* hpredecdata = nullptr;

    try {
        // generate preprocessor maps
        std::map<std::string, byte> prepro_name2num = getPreproMap();
        std::string prepro_num2name [256];
        for (auto pair: prepro_name2num) {
        prepro_num2name[pair.second] = pair.first;
        }

        // generate component maps
        std::map<std::string, byte> comp_name2num = getCompMap();
        std::string comp_num2name [256];
        for (auto pair: comp_name2num) {
        comp_num2name[pair.second] = pair.first;
        }

        int stages;
        unsigned long long algorithms;
        std::vector<std::pair<byte, std::vector<double>>> prepros;
        std::vector<std::vector<byte>> comp_list;

        auto prepros_str = std::string(prepros_cstr);
        auto comp_str = std::string(comp_cstr);

        prepros = getItems(prepro_name2num, prepros_str.data());
        comp_list = getStages(comp_name2num, comp_str.data(), stages, algorithms);
        if (algorithms != 1) {
            fprintf(stderr, "ERROR: pipeline must describe one algorithm\n\n");
            throw std::runtime_error("LC error");
        }

        printStages(prepros, prepro_name2num, comp_list, comp_name2num, stages, algorithms);

        printf("encoded size: %lld bytes\n\n", encsize);

        long long pre_size = 0;
        assert(encsize >= sizeof(pre_size));
        std::memcpy(&pre_size, encoded, sizeof(pre_size));

        // allocate CPU memory
        byte* hdecoded = new byte [pre_size];
        long long hdecsize = 0;

        // create chain for current combination and output
        unsigned long long combin = 0;
        unsigned long long chain = 0;
        for (int s = 0; s < stages; s++) {
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

        delete [] hdecoded;

        *decoded = hpredecdata;
        *decsize = hpredecsize;

        return 0;
    }
    catch(const std::exception &ex) {
        std::cerr << ex.what() << std::endl;

        if (hdecoded != nullptr) delete [] hdecoded;
        if (hpredecdata != nullptr) delete [] hpredecdata;

        return -1;
    }
}
