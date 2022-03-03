# IBC Protocol

appchain 资产的lock， 也就是transfer到一个pallet控制的账号里
https://github.com/octopus-network/octopus-pallets/blob/main/appchain/src/lib.rs#L694
基于pallet-assets的资产可以burn
https://github.com/octopus-network/octopus-pallets/blob/main/appchain/src/lib.rs#L753
unlock就是从pallet控制的账号中将资产transfer出来
https://github.com/octopus-network/octopus-pallets/blob/main/appchain/src/lib.rs#L1062
基于pallet-asssets的资产可以mint
https://github.com/octopus-network/octopus-pallets/blob/main/appchain/src/lib.rs#L1078