/**

Generated by the following Solidity interface...
```solidity
interface GetUniswapV2PairsBatchRequest {
    constructor(uint256 from, uint256 step, address factory);
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [
      {
        "name": "from",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "step",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "factory",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "nonpayable"
  }
]
```*/
#[allow(non_camel_case_types, non_snake_case, clippy::style)]
pub mod GetUniswapV2PairsBatchRequest {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x608060405234801561001057600080fd5b506040516104c33803806104c383398181016040528101906100329190610243565b6000838361004091906102c5565b905060008167ffffffffffffffff81111561005e5761005d6102f9565b5b60405190808252806020026020018201604052801561008c5781602001602082028036833780820191505090505b50905060005b8281101561017b578373ffffffffffffffffffffffffffffffffffffffff16631e3dd18b82886100c29190610328565b6040518263ffffffff1660e01b81526004016100de919061036b565b6020604051808303816000875af11580156100fd573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906101219190610386565b828281518110610134576101336103b3565b5b602002602001019073ffffffffffffffffffffffffffffffffffffffff16908173ffffffffffffffffffffffffffffffffffffffff16815250508080600101915050610092565b5060008160405160200161018f91906104a0565b60405160208183030381529060405290506020810180590381f35b600080fd5b6000819050919050565b6101c2816101af565b81146101cd57600080fd5b50565b6000815190506101df816101b9565b92915050565b600073ffffffffffffffffffffffffffffffffffffffff82169050919050565b6000610210826101e5565b9050919050565b61022081610205565b811461022b57600080fd5b50565b60008151905061023d81610217565b92915050565b60008060006060848603121561025c5761025b6101aa565b5b600061026a868287016101d0565b935050602061027b868287016101d0565b925050604061028c8682870161022e565b9150509250925092565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052601160045260246000fd5b60006102d0826101af565b91506102db836101af565b92508282039050818111156102f3576102f2610296565b5b92915050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052604160045260246000fd5b6000610333826101af565b915061033e836101af565b925082820190508082111561035657610355610296565b5b92915050565b610365816101af565b82525050565b6000602082019050610380600083018461035c565b92915050565b60006020828403121561039c5761039b6101aa565b5b60006103aa8482850161022e565b91505092915050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052603260045260246000fd5b600081519050919050565b600082825260208201905092915050565b6000819050602082019050919050565b61041781610205565b82525050565b6000610429838361040e565b60208301905092915050565b6000602082019050919050565b600061044d826103e2565b61045781856103ed565b9350610462836103fe565b8060005b8381101561049357815161047a888261041d565b975061048583610435565b925050600181019050610466565b5085935050505092915050565b600060208201905081810360008301526104ba8184610442565b90509291505056fe
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x04\xC38\x03\x80a\x04\xC3\x839\x81\x81\x01`@R\x81\x01\x90a\x002\x91\x90a\x02CV[`\0\x83\x83a\0@\x91\x90a\x02\xC5V[\x90P`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\0^Wa\0]a\x02\xF9V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\0\x8CW\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P`\0[\x82\x81\x10\x15a\x01{W\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x1E=\xD1\x8B\x82\x88a\0\xC2\x91\x90a\x03(V[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\0\xDE\x91\x90a\x03kV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\0\xFDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01!\x91\x90a\x03\x86V[\x82\x82\x81Q\x81\x10a\x014Wa\x013a\x03\xB3V[[` \x02` \x01\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x80\x80`\x01\x01\x91PPa\0\x92V[P`\0\x81`@Q` \x01a\x01\x8F\x91\x90a\x04\xA0V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P` \x81\x01\x80Y\x03\x81\xF3[`\0\x80\xFD[`\0\x81\x90P\x91\x90PV[a\x01\xC2\x81a\x01\xAFV[\x81\x14a\x01\xCDW`\0\x80\xFD[PV[`\0\x81Q\x90Pa\x01\xDF\x81a\x01\xB9V[\x92\x91PPV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0a\x02\x10\x82a\x01\xE5V[\x90P\x91\x90PV[a\x02 \x81a\x02\x05V[\x81\x14a\x02+W`\0\x80\xFD[PV[`\0\x81Q\x90Pa\x02=\x81a\x02\x17V[\x92\x91PPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x02\\Wa\x02[a\x01\xAAV[[`\0a\x02j\x86\x82\x87\x01a\x01\xD0V[\x93PP` a\x02{\x86\x82\x87\x01a\x01\xD0V[\x92PP`@a\x02\x8C\x86\x82\x87\x01a\x02.V[\x91PP\x92P\x92P\x92V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0a\x02\xD0\x82a\x01\xAFV[\x91Pa\x02\xDB\x83a\x01\xAFV[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15a\x02\xF3Wa\x02\xF2a\x02\x96V[[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`\0a\x033\x82a\x01\xAFV[\x91Pa\x03>\x83a\x01\xAFV[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a\x03VWa\x03Ua\x02\x96V[[\x92\x91PPV[a\x03e\x81a\x01\xAFV[\x82RPPV[`\0` \x82\x01\x90Pa\x03\x80`\0\x83\x01\x84a\x03\\V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x03\x9CWa\x03\x9Ba\x01\xAAV[[`\0a\x03\xAA\x84\x82\x85\x01a\x02.V[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\0\x81Q\x90P\x91\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0\x81\x90P` \x82\x01\x90P\x91\x90PV[a\x04\x17\x81a\x02\x05V[\x82RPPV[`\0a\x04)\x83\x83a\x04\x0EV[` \x83\x01\x90P\x92\x91PPV[`\0` \x82\x01\x90P\x91\x90PV[`\0a\x04M\x82a\x03\xE2V[a\x04W\x81\x85a\x03\xEDV[\x93Pa\x04b\x83a\x03\xFEV[\x80`\0[\x83\x81\x10\x15a\x04\x93W\x81Qa\x04z\x88\x82a\x04\x1DV[\x97Pa\x04\x85\x83a\x045V[\x92PP`\x01\x81\x01\x90Pa\x04fV[P\x85\x93PPPP\x92\x91PPV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x04\xBA\x81\x84a\x04BV[\x90P\x92\x91PPV\xFE",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x6080604052600080fdfea2646970667358221220728220861ca2785f2660024a836d49def26cd9223f9e6747109e02b8b976f31364736f6c634300081a0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\0\x80\xFD\xFE\xA2dipfsX\"\x12 r\x82 \x86\x1C\xA2x_&`\x02J\x83mI\xDE\xF2l\xD9\"?\x9EgG\x10\x9E\x02\xB8\xB9v\xF3\x13dsolcC\0\x08\x1A\x003",
    );
    /**Constructor`.
```solidity
constructor(uint256 from, uint256 step, address factory);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct constructorCall {
        pub from: alloy::sol_types::private::U256,
        pub step: alloy::sol_types::private::U256,
        pub factory: alloy::sol_types::private::Address,
    }
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::U256,
                alloy::sol_types::private::U256,
                alloy::sol_types::private::Address,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    (value.from, value.step, value.factory)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        from: tuple.0,
                        step: tuple.1,
                        factory: tuple.2,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.from),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.step),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.factory,
                    ),
                )
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`GetUniswapV2PairsBatchRequest`](self) contract instance.

See the [wrapper's documentation](`GetUniswapV2PairsBatchRequestInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> GetUniswapV2PairsBatchRequestInstance<T, P, N> {
        GetUniswapV2PairsBatchRequestInstance::<T, P, N>::new(address, provider)
    }
    /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
    #[inline]
    pub fn deploy<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        provider: P,
        from: alloy::sol_types::private::U256,
        step: alloy::sol_types::private::U256,
        factory: alloy::sol_types::private::Address,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<GetUniswapV2PairsBatchRequestInstance<T, P, N>>,
    > {
        GetUniswapV2PairsBatchRequestInstance::<
            T,
            P,
            N,
        >::deploy(provider, from, step, factory)
    }
    /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
    #[inline]
    pub fn deploy_builder<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        provider: P,
        from: alloy::sol_types::private::U256,
        step: alloy::sol_types::private::U256,
        factory: alloy::sol_types::private::Address,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        GetUniswapV2PairsBatchRequestInstance::<
            T,
            P,
            N,
        >::deploy_builder(provider, from, step, factory)
    }
    /**A [`GetUniswapV2PairsBatchRequest`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`GetUniswapV2PairsBatchRequest`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct GetUniswapV2PairsBatchRequestInstance<
        T,
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for GetUniswapV2PairsBatchRequestInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("GetUniswapV2PairsBatchRequestInstance")
                .field(&self.address)
                .finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > GetUniswapV2PairsBatchRequestInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`GetUniswapV2PairsBatchRequest`](self) contract instance.

See the [wrapper's documentation](`GetUniswapV2PairsBatchRequestInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
            }
        }
        /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
        #[inline]
        pub async fn deploy(
            provider: P,
            from: alloy::sol_types::private::U256,
            step: alloy::sol_types::private::U256,
            factory: alloy::sol_types::private::Address,
        ) -> alloy_contract::Result<GetUniswapV2PairsBatchRequestInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(provider, from, step, factory);
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(
            provider: P,
            from: alloy::sol_types::private::U256,
            step: alloy::sol_types::private::U256,
            factory: alloy::sol_types::private::Address,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall {
                            from,
                            step,
                            factory,
                        },
                    )[..],
                ]
                    .concat()
                    .into(),
            )
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<T, P: ::core::clone::Clone, N> GetUniswapV2PairsBatchRequestInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(
            self,
        ) -> GetUniswapV2PairsBatchRequestInstance<T, P, N> {
            GetUniswapV2PairsBatchRequestInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > GetUniswapV2PairsBatchRequestInstance<T, P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > GetUniswapV2PairsBatchRequestInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
