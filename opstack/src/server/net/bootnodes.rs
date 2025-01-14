use std::str::FromStr;

use discv5::enr::{CombinedKey, Enr};

/// Default bootnodes to use. Currently consists of 2 Base bootnodes & 1 Op Mainnet bootnode.
pub fn bootnodes() -> Vec<Enr<CombinedKey>> {
    let bootnodes = [
        "enr:-J64QBbwPjPLZ6IOOToOLsSjtFUjjzN66qmBZdUexpO32Klrc458Q24kbty2PdRaLacHM5z-cZQr8mjeQu3pik6jPSOGAYYFIqBfgmlkgnY0gmlwhDaRWFWHb3BzdGFja4SzlAUAiXNlY3AyNTZrMaECmeSnJh7zjKrDSPoNMGXoopeDF4hhpj5I0OsQUUt4u8uDdGNwgiQGg3VkcIIkBg",
        "enr:-J64QAlTCDa188Hl1OGv5_2Kj2nWCsvxMVc_rEnLtw7RPFbOfqUOV6khXT_PH6cC603I2ynY31rSQ8sI9gLeJbfFGaWGAYYFIrpdgmlkgnY0gmlwhANWgzCHb3BzdGFja4SzlAUAiXNlY3AyNTZrMaECkySjcg-2v0uWAsFsZZu43qNHppGr2D5F913Qqs5jDCGDdGNwgiQGg3VkcIIkBg",
        "enr:-J24QGEzN4mJgLWNTUNwj7riVJ2ZjRLenOFccl2dbRFxHHOCCZx8SXWzgf-sLzrGs6QgqSFCvGXVgGPBkRkfOWlT1-iGAYe6Cu93gmlkgnY0gmlwhCJBEUSHb3BzdGFja4OkAwCJc2VjcDI1NmsxoQLuYIwaYOHg3CUQhCkS-RsSHmUd1b_x93-9yQ5ItS6udIN0Y3CCIyuDdWRwgiMr",

        //Base bootnodes
        "enr:-J24QNz9lbrKbN4iSmmjtnr7SjUMk4zB7f1krHZcTZx-JRKZd0kA2gjufUROD6T3sOWDVDnFJRvqBBo62zuF-hYCohOGAYiOoEyEgmlkgnY0gmlwhAPniryHb3BzdGFja4OFQgCJc2VjcDI1NmsxoQKNVFlCxh_B-716tTs-h1vMzZkSs1FTu_OYTNjgufplG4N0Y3CCJAaDdWRwgiQG",
        "enr:-J24QH-f1wt99sfpHy4c0QJM-NfmsIfmlLAMMcgZCUEgKG_BBYFc6FwYgaMJMQN5dsRBJApIok0jFn-9CS842lGpLmqGAYiOoDRAgmlkgnY0gmlwhLhIgb2Hb3BzdGFja4OFQgCJc2VjcDI1NmsxoQJ9FTIv8B9myn1MWaC_2lJ-sMoeCDkusCsk4BYHjjCq04N0Y3CCJAaDdWRwgiQG",
        "enr:-J24QDXyyxvQYsd0yfsN0cRr1lZ1N11zGTplMNlW4xNEc7LkPXh0NAJ9iSOVdRO95GPYAIc6xmyoCCG6_0JxdL3a0zaGAYiOoAjFgmlkgnY0gmlwhAPckbGHb3BzdGFja4OFQgCJc2VjcDI1NmsxoQJwoS7tzwxqXSyFL7g0JM-KWVbgvjfB8JA__T7yY_cYboN0Y3CCJAaDdWRwgiQG",
        "enr:-J24QHmGyBwUZXIcsGYMaUqGGSl4CFdx9Tozu-vQCn5bHIQbR7On7dZbU61vYvfrJr30t0iahSqhc64J46MnUO2JvQaGAYiOoCKKgmlkgnY0gmlwhAPnCzSHb3BzdGFja4OFQgCJc2VjcDI1NmsxoQINc4fSijfbNIiGhcgvwjsjxVFJHUstK9L1T8OTKUjgloN0Y3CCJAaDdWRwgiQG",
        "enr:-J24QG3ypT4xSu0gjb5PABCmVxZqBjVw9ca7pvsI8jl4KATYAnxBmfkaIuEqy9sKvDHKuNCsy57WwK9wTt2aQgcaDDyGAYiOoGAXgmlkgnY0gmlwhDbGmZaHb3BzdGFja4OFQgCJc2VjcDI1NmsxoQIeAK_--tcLEiu7HvoUlbV52MspE0uCocsx1f_rYvRenIN0Y3CCJAaDdWRwgiQG",

        //op-sepolia
        "enr:-J-4QOL3ttp_Ll2eKHtoFJibDo_CVDMBLgTLxIgSILT0TuAMMuS49UWI_eVFRa7WaOdn9iQy3F4F-PZHm6AhDzSj77uGAZQ75qCSgmlkgnY0gmlwhAPv7d2Hb3BzdGFja4Xc76gFAIlzZWNwMjU2azGhAx8AvZKOdJNTckIsHbTxDrOlY80Z66j1V9_ginpyvsWIg3RjcIJ2X4N1ZHCCdl8",
        "enr:-J-4QIQUt39MfkrXaTmZx0yx7vJDkTOgDbCg5ilnVMGmSB7ZUK7xseTcrkrz9t4aJZugVR07uwgjo5uPSJrX7hzFHi-GAZQw_tcIgmlkgnY0gmlwhD_76H2Hb3BzdGFja4Xc76gFAIlzZWNwMjU2azGhAmqwHQhhCrp2dzzfN_RAyPeNkkMHhUZUUBYaOU8SS4yzg3RjcIJ2nYN1ZHCCdp0",
        "enr:-J-4QF9wqT2uYd135WKa22q2JL3BWRg7MirLoQfikhyRE2klc5LE6Zw9tWCvJAV4OXAs8yXarazsqB5jYXdcPUkvDp2GAZQnL788gmlkgnY0gmlwhANdMTOHb3BzdGFja4Xc76gFAIlzZWNwMjU2azGhAnvWZyTGnVzMAOaGM-6Fn_TQnDZzeCu0O8FGR9DT9rMFg3RjcIJ2X4N1ZHCCdl8",
        "enr:-J-4QEGiFAFH9AmAvoKxCiElDhCwTCmIgwhA2CINPUJHEONydYzShrsWqgMI3QdKceeuH8V2DOHkPExOwskjUqTWMj-GAZPPjjdwgmlkgnY0gmlwhCPMrnyHb3BzdGFja4Xc76gFAIlzZWNwMjU2azGhAzQz2hFMgXnOrTXrCR8xSTBqqaoXw8D_Ye-qfk4dbgVRg3RjcIIjK4N1ZHCCIys",
        "enr:-J-4QNwNZ5ZVgLh5OeHD8o9AftIKyUqZrZGVYBO_vJCYGrwWGRnbs-Flg1OjvyBTvn7g1ti4i4ht32u0rA2QGa2GTjCGAZPWc2YXgmlkgnY0gmlwhCIiMRiHb3BzdGFja4Xc76gFAIlzZWNwMjU2azGhAsydnpPXZ8_Tu6UfYyIp5LRPvWKGOTWH2qGsger3zCMBg3RjcIIjK4N1ZHCCIys",
        "enr:-J-4QHGdix-Z6JVfGMg7738Z23ugZDRhOvJB_we_MyPqRJMeL1o88lE5hhh7_mrqOcRnD-mOWRQzbq8eWzuzl4YDb2WGAZNrfGcCgmlkgnY0gmlwhI6E-XCHb3BzdGFja4Xc76gFAIlzZWNwMjU2azGhAgI2UtCPzV42z4t70n5TDkmN01hjzkEO5g_0q-_aq-eUg3RjcIJU1YN1ZHCCONE",
        "enr:-J-4QEC2ziNrKzbvTEjHT345IF8cHVOIVBvDPn9Ed-Mu5mstUpqAqyMmtixz1q6b4_A_zMJktOZDt5qWIdfslFszwayGAZPVnsakgmlkgnY0gmlwhCIglcCHb3BzdGFja4Xc76gFAIlzZWNwMjU2azGhAp4Zv5lcyV5wYx0BrWjUu4K0i-QCp59xlGbdQCv_IHHrg3RjcIIjK4N1ZHCCIys",
        "enr:-J-4QOkqfrmiTGgFFQVTDSsUsrtwUUD1sA9Y3dHe0dWVLRwrI23Edq_kBge0dNtJxFVJZwmxZl3HDqQR53Cs4cCMqzGGAZKgBsWCgmlkgnY0gmlwhCW7jF-Hb3BzdGFja4Xc76gFAIlzZWNwMjU2azGhAr0dzfAhuZvDhw0ufAYeXSt84J0XFm_UTUy0CXyfn1MRg3RjcIIkBoN1ZHCC2Xk",    
    ];

    bootnodes
        .iter()
        .filter_map(|enr| Enr::from_str(enr).ok())
        .collect()
}
