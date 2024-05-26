use super::{Node, NodeCodegen};
use crate::burn::{TensorKind, TensorType, ToTokens, Type};

use burn::record::PrecisionSettings;
use quote::quote;

#[derive(Debug, Clone, new)]
pub struct ArgMaxNode {
    pub input: TensorType,
    pub output: TensorType,
    pub axis: usize,
    pub select_last_index: usize,
    pub keepdims: usize,
}

impl<PS: PrecisionSettings> NodeCodegen<PS> for ArgMaxNode {
    fn output_types(&self) -> Vec<Type> {
        let mut output = self.output.clone();
        output.kind = TensorKind::Int;
        vec![Type::Tensor(output)]
    }

    fn input_types(&self) -> Vec<crate::burn::Type> {
        vec![Type::Tensor(self.input.clone())]
    }

    fn forward(
        &self,
        scope: &mut crate::burn::Scope,
        node_position: usize,
    ) -> proc_macro2::TokenStream {
        let axis = self.axis.to_tokens();

        //NOTE: are select_last_index and keep_dims supported?
        let _select_last_index = self.select_last_index.to_tokens();
        let _keepdims = self.keepdims.to_tokens();
        let input = scope.tensor_use_owned(&self.input, node_position);
        let output = &self.output.name;

        quote! {
            let #output = #input.argmax(#axis);
        }
    }

    fn into_node(self) -> super::Node<PS> {
        Node::ArgMax(self)
    }
}

#[cfg(test)]
mod tests {

    use burn::record::FullPrecisionSettings;

    use super::*;
    use crate::burn::{graph::BurnGraph, node::test::assert_tokens, TensorType};

    #[test]
    fn test_codegen_gather() {
        let mut graph = BurnGraph::<FullPrecisionSettings>::default();

        graph.register(ArgMaxNode::new(
            TensorType::new_float("tensor1", 2),
            TensorType::new_float("tensor2", 2),
            1,
            0,
            0,
        ));

        graph.register_input_output(vec!["tensor1".to_string()], vec!["tensor2".to_string()]);

        let expected = quote! {
            use burn::tensor::Int;
            use burn::{
                module::Module,
                tensor::{backend::Backend, Tensor},
            };

            #[derive(Module, Debug)]
            pub struct Model<B: Backend> {
                phantom: core::marker::PhantomData<B>,
                device: burn::module::Ignored<B::Device>,
            }

            impl<B: Backend> Model <B> {
                #[allow(unused_variables)]
                pub fn new(device: &B::Device) -> Self {
                    Self {
                        phantom: core::marker::PhantomData,
                        device: burn::module::Ignored(device.clone()),
                    }
                }

                #[allow(clippy::let_and_return, clippy::approx_constant)]
                pub fn forward(
                    &self,
                    tensor1: Tensor<B, 2>
                ) -> Tensor<B, 2, Int> {
                    let tensor2 = tensor1.argmax(1);

                    tensor2
                }
            }
        };

        assert_tokens(graph.codegen(), expected);
    }
}
