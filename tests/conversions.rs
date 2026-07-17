use caseidae::{
    ToAttributeName, ToConstantName, ToCrateName, ToFieldName, ToFunctionName, ToLifetimeName,
    ToMacroName, ToModuleName, ToPrimitiveName, ToStaticName, ToTraitName, ToTypeName,
    ToVariableName, ToVariantName,
};
#[cfg(feature = "syn")]
use proc_macro2::Span;
use std::fmt::Debug;
#[cfg(feature = "syn")]
use syn::Ident;

#[derive(Clone)]
enum TestCasing {
    Pascal,
    Snake,
    ScreamingSnake,
    Flat,
    LifetimeName,
}

struct TestCase {
    input: &'static str,
    output_pascal_case: &'static str,
    output_snake_case: &'static str,
    output_screaming_snake_case: &'static str,
    output_flat_case: &'static str,
}

impl TestCase {
    const fn new(
        input: &'static str,
        output_pascal_case: &'static str,
        output_snake_case: &'static str,
        output_screaming_snake_case: &'static str,
        output_primitive_name: &'static str,
    ) -> Self {
        Self {
            input,
            output_pascal_case,
            output_snake_case,
            output_screaming_snake_case,
            output_flat_case: output_primitive_name,
        }
    }

    fn output_lifetime_name(&self) -> String {
        format!("'{}", self.output_snake_case)
    }

    fn select_output(&self, casing: TestCasing) -> String {
        match casing {
            TestCasing::Pascal => self.output_pascal_case.to_owned(),
            TestCasing::Snake => self.output_snake_case.to_owned(),
            TestCasing::ScreamingSnake => self.output_screaming_snake_case.to_owned(),
            TestCasing::Flat => self.output_flat_case.to_owned(),
            TestCasing::LifetimeName => self.output_lifetime_name(),
        }
    }

    fn test<Output: ToString + Debug, Converter: Fn(&str) -> Output>(
        &self,
        converter: Converter,
        casing: TestCasing,
    ) {
        assert_eq!(
            converter(self.input).to_string(),
            *self.select_output(casing).as_str()
        );
    }
}

struct TestCases<const COUNT: usize>([TestCase; COUNT]);

const TEST_CASES: TestCases<43> = TestCases([
    // Pascal case inputs.
    TestCase::new(
        "AlexAndDerMurder",
        "AlexAndDerMurder",
        "alex_and_der_murder",
        "ALEX_AND_DER_MURDER",
        "alexanddermurder",
    ),
    TestCase::new(
        "AlexANDDerMurdeR",
        "AlexANDDerMurdeR",
        "alex_a_n_d_der_murde_r",
        "ALEX_A_N_D_DER_MURDE_R",
        "alexanddermurder",
    ),
    TestCase::new("T", "T", "t", "T", "t"),
    TestCase::new(
        "Escape2Africa",
        "Escape2Africa",
        "escape_2_africa",
        "ESCAPE_2_AFRICA",
        "escape2africa",
    ),
    TestCase::new(
        "X86VsRisc5",
        "X86VsRisc5",
        "x_86_vs_risc_5",
        "X_86_VS_RISC_5",
        "x86vsrisc5",
    ),
    TestCase::new(
        "r#AlexAndDerMurder",
        "r#AlexAndDerMurder",
        "r#alex_and_der_murder",
        "r#ALEX_AND_DER_MURDER",
        "r#alexanddermurder",
    ),
    TestCase::new(
        "_AlexAndDerMurder",
        "_AlexAndDerMurder",
        "_alex_and_der_murder",
        "_ALEX_AND_DER_MURDER",
        "_alexanddermurder",
    ),
    TestCase::new(
        "AlexAndDerMurder_",
        "AlexAndDerMurder_",
        "alex_and_der_murder_",
        "ALEX_AND_DER_MURDER_",
        "alexanddermurder_",
    ),
    TestCase::new(
        "___AlexAndDerMurder__",
        "___AlexAndDerMurder__",
        "___alex_and_der_murder__",
        "___ALEX_AND_DER_MURDER__",
        "___alexanddermurder__",
    ),
    TestCase::new(
        "r#__AlexAndDerMurder",
        "r#__AlexAndDerMurder",
        "r#__alex_and_der_murder",
        "r#__ALEX_AND_DER_MURDER",
        "r#__alexanddermurder",
    ),
    // Snake case inputs.
    TestCase::new(
        "alex_and_der_murder",
        "AlexAndDerMurder",
        "alex_and_der_murder",
        "ALEX_AND_DER_MURDER",
        "alexanddermurder",
    ),
    TestCase::new(
        "alex_a_n_d_der_murde_r",
        "AlexANDDerMurdeR",
        "alex_a_n_d_der_murde_r",
        "ALEX_A_N_D_DER_MURDE_R",
        "alexanddermurder",
    ),
    TestCase::new("t", "T", "t", "T", "t"),
    TestCase::new(
        "escape_2_africa",
        "Escape2Africa",
        "escape_2_africa",
        "ESCAPE_2_AFRICA",
        "escape2africa",
    ),
    TestCase::new(
        "x_86_vs_risc_5",
        "X86VsRisc5",
        "x_86_vs_risc_5",
        "X_86_VS_RISC_5",
        "x86vsrisc5",
    ),
    TestCase::new(
        "r#alex_and_der_murder",
        "r#AlexAndDerMurder",
        "r#alex_and_der_murder",
        "r#ALEX_AND_DER_MURDER",
        "r#alexanddermurder",
    ),
    TestCase::new(
        "_alex_and_der_murder",
        "_AlexAndDerMurder",
        "_alex_and_der_murder",
        "_ALEX_AND_DER_MURDER",
        "_alexanddermurder",
    ),
    TestCase::new(
        "alex_and_der_murder_",
        "AlexAndDerMurder_",
        "alex_and_der_murder_",
        "ALEX_AND_DER_MURDER_",
        "alexanddermurder_",
    ),
    TestCase::new(
        "___alex_and_der_murder__",
        "___AlexAndDerMurder__",
        "___alex_and_der_murder__",
        "___ALEX_AND_DER_MURDER__",
        "___alexanddermurder__",
    ),
    TestCase::new(
        "r#__alex_and_der_murder",
        "r#__AlexAndDerMurder",
        "r#__alex_and_der_murder",
        "r#__ALEX_AND_DER_MURDER",
        "r#__alexanddermurder",
    ),
    // Screaming snake case inputs.
    TestCase::new(
        "ALEX_AND_DER_MURDER",
        "AlexAndDerMurder",
        "alex_and_der_murder",
        "ALEX_AND_DER_MURDER",
        "alexanddermurder",
    ),
    TestCase::new(
        "ALEX_A_N_D_DER_MURDE_R",
        "AlexANDDerMurdeR",
        "alex_a_n_d_der_murde_r",
        "ALEX_A_N_D_DER_MURDE_R",
        "alexanddermurder",
    ),
    TestCase::new(
        "ESCAPE_2_AFRICA",
        "Escape2Africa",
        "escape_2_africa",
        "ESCAPE_2_AFRICA",
        "escape2africa",
    ),
    TestCase::new(
        "X_86_VS_RISC_5",
        "X86VsRisc5",
        "x_86_vs_risc_5",
        "X_86_VS_RISC_5",
        "x86vsrisc5",
    ),
    TestCase::new(
        "r#ALEX_AND_DER_MURDER",
        "r#AlexAndDerMurder",
        "r#alex_and_der_murder",
        "r#ALEX_AND_DER_MURDER",
        "r#alexanddermurder",
    ),
    TestCase::new(
        "_ALEX_AND_DER_MURDER",
        "_AlexAndDerMurder",
        "_alex_and_der_murder",
        "_ALEX_AND_DER_MURDER",
        "_alexanddermurder",
    ),
    TestCase::new(
        "ALEX_AND_DER_MURDER_",
        "AlexAndDerMurder_",
        "alex_and_der_murder_",
        "ALEX_AND_DER_MURDER_",
        "alexanddermurder_",
    ),
    TestCase::new(
        "___ALEX_AND_DER_MURDER__",
        "___AlexAndDerMurder__",
        "___alex_and_der_murder__",
        "___ALEX_AND_DER_MURDER__",
        "___alexanddermurder__",
    ),
    TestCase::new(
        "r#__ALEX_AND_DER_MURDER",
        "r#__AlexAndDerMurder",
        "r#__alex_and_der_murder",
        "r#__ALEX_AND_DER_MURDER",
        "r#__alexanddermurder",
    ),
    // Primitive name inputs.
    TestCase::new(
        "alexanddermurder",
        "Alexanddermurder",
        "alexanddermurder",
        "ALEXANDDERMURDER",
        "alexanddermurder",
    ),
    TestCase::new(
        "escape2africa",
        "Escape2Africa",
        "escape_2_africa",
        "ESCAPE_2_AFRICA",
        "escape2africa",
    ),
    TestCase::new(
        "x86vsrisc5",
        "X86Vsrisc5",
        "x_86_vsrisc_5",
        "X_86_VSRISC_5",
        "x86vsrisc5",
    ),
    TestCase::new(
        "r#alexanddermurder",
        "r#Alexanddermurder",
        "r#alexanddermurder",
        "r#ALEXANDDERMURDER",
        "r#alexanddermurder",
    ),
    TestCase::new(
        "_alexanddermurder",
        "_Alexanddermurder",
        "_alexanddermurder",
        "_ALEXANDDERMURDER",
        "_alexanddermurder",
    ),
    TestCase::new(
        "alexanddermurder_",
        "Alexanddermurder_",
        "alexanddermurder_",
        "ALEXANDDERMURDER_",
        "alexanddermurder_",
    ),
    TestCase::new(
        "___alexanddermurder__",
        "___Alexanddermurder__",
        "___alexanddermurder__",
        "___ALEXANDDERMURDER__",
        "___alexanddermurder__",
    ),
    TestCase::new(
        "r#__alexanddermurder",
        "r#__Alexanddermurder",
        "r#__alexanddermurder",
        "r#__ALEXANDDERMURDER",
        "r#__alexanddermurder",
    ),
    // Combinations.
    TestCase::new(
        "Alex_AND_DerMurder",
        "AlexANDDerMurder",
        "alex_a_n_d_der_murder",
        "ALEX_A_N_D_DER_MURDER",
        "alexanddermurder",
    ),
    // Edgecases.
    TestCase::new(
        "__alex__and___der_murder___",
        "__AlexAndDerMurder___",
        "__alex_and_der_murder___",
        "__ALEX_AND_DER_MURDER___",
        "__alexanddermurder___",
    ),
    TestCase::new(
        "_2025_08_01t12Hh00MS0_",
        "_20250801T12Hh00MS0_",
        "_2025_08_01_t_12_hh_00_m_s_0_",
        "_2025_08_01_T_12_HH_00_M_S_0_",
        "_20250801t12hh00ms0_",
    ),
    TestCase::new("_", "_", "_", "_", "_"),
    TestCase::new("__", "__", "__", "__", "__"),
    TestCase::new("___", "___", "___", "___", "___"),
]);
/// Invalid identifiers.
const TEST_CASES_STR_ONLY: TestCases<21> = TestCases([
    // Lifetime name inputs.
    TestCase::new(
        "'alex_and_der_murder",
        "AlexAndDerMurder",
        "alex_and_der_murder",
        "ALEX_AND_DER_MURDER",
        "alexanddermurder",
    ),
    TestCase::new(
        "'alex_a_n_d_der_murder",
        "AlexANDDerMurder",
        "alex_a_n_d_der_murder",
        "ALEX_A_N_D_DER_MURDER",
        "alexanddermurder",
    ),
    TestCase::new("t", "T", "t", "T", "t"),
    TestCase::new(
        "'escape_2_africa",
        "Escape2Africa",
        "escape_2_africa",
        "ESCAPE_2_AFRICA",
        "escape2africa",
    ),
    TestCase::new(
        "'x_86_vs_risc_5",
        "X86VsRisc5",
        "x_86_vs_risc_5",
        "X_86_VS_RISC_5",
        "x86vsrisc5",
    ),
    TestCase::new(
        "'r#alex_and_der_murder",
        "r#AlexAndDerMurder",
        "r#alex_and_der_murder",
        "r#ALEX_AND_DER_MURDER",
        "r#alexanddermurder",
    ),
    TestCase::new(
        "'_alex_and_der_murder",
        "_AlexAndDerMurder",
        "_alex_and_der_murder",
        "_ALEX_AND_DER_MURDER",
        "_alexanddermurder",
    ),
    TestCase::new(
        "'alex_and_der_murder_",
        "AlexAndDerMurder_",
        "alex_and_der_murder_",
        "ALEX_AND_DER_MURDER_",
        "alexanddermurder_",
    ),
    TestCase::new(
        "'___alex_and_der_murder__",
        "___AlexAndDerMurder__",
        "___alex_and_der_murder__",
        "___ALEX_AND_DER_MURDER__",
        "___alexanddermurder__",
    ),
    TestCase::new(
        "'r#__alex_and_der_murder",
        "r#__AlexAndDerMurder",
        "r#__alex_and_der_murder",
        "r#__ALEX_AND_DER_MURDER",
        "r#__alexanddermurder",
    ),
    // Edgecases.
    TestCase::new(
        " Alex and   der  murder  ",
        "AlexAndDerMurder",
        "alex_and_der_murder",
        "ALEX_AND_DER_MURDER",
        "alexanddermurder",
    ),
    TestCase::new(
        "alexAnd_der#murder#",
        "AlexAndDerMurder",
        "alex_and_der_murder",
        "ALEX_AND_DER_MURDER",
        "alexanddermurder",
    ),
    TestCase::new(" r#a", "RA", "r_a", "R_A", "ra"),
    TestCase::new("r#r#r#oar", "r#RROar", "r#r_r_oar", "r#R_R_OAR", "r#rroar"),
    TestCase::new("_ ", "_", "_", "_", "_"),
    TestCase::new(" _", "_", "_", "_", "_"),
    TestCase::new(" _a", "A", "a", "A", "a"),
    TestCase::new("a_ ", "A", "a", "A", "a"),
    TestCase::new("r#_", "r#_", "r#_", "r#_", "r#_"),
    TestCase::new("r#__", "r#__", "r#__", "r#__", "r#__"),
    TestCase::new(
        "https://hryziko.dev",
        "HttpsHryzikoDev",
        "https_hryziko_dev",
        "HTTPS_HRYZIKO_DEV",
        "httpshryzikodev",
    ),
]);

impl<const COUNT: usize> TestCases<COUNT> {
    fn test<Output: ToString + Debug>(
        &self,
        converter: impl Fn(&str) -> Output,
        casing: TestCasing,
    ) {
        for test_case in self.0.iter() {
            test_case.test(&converter, casing.clone());
        }
    }
}

#[cfg(feature = "syn")]
use std::{convert::Infallible, str::FromStr};

#[cfg(feature = "syn")]
struct TestIdent(Ident);

#[cfg(feature = "syn")]
impl TestIdent {
    fn convert<Output>(input: &str, converter: impl Fn(&Ident) -> Output) -> Output {
        let Ok(input) = TestIdent::from_str(input);
        converter(&input.0)
    }
}

#[cfg(feature = "syn")]
impl FromStr for TestIdent {
    type Err = Infallible;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let ident = if let Some(rest) = value.strip_prefix("r#") {
            Ident::new_raw(rest, Span::call_site())
        } else {
            Ident::new(value, Span::call_site())
        };
        Ok(Self(ident))
    }
}

#[test]
fn to_attribute_name() {
    TEST_CASES.test(str::to_attribute_name, TestCasing::Snake);
    TEST_CASES_STR_ONLY.test(str::to_attribute_name, TestCasing::Snake);
    #[cfg(feature = "syn")]
    TEST_CASES.test(
        |input| TestIdent::convert(input, Ident::to_attribute_name),
        TestCasing::Snake,
    );
}

#[test]
fn to_constant_name() {
    TEST_CASES.test(str::to_constant_name, TestCasing::ScreamingSnake);
    TEST_CASES_STR_ONLY.test(str::to_constant_name, TestCasing::ScreamingSnake);
    #[cfg(feature = "syn")]
    TEST_CASES.test(
        |input| TestIdent::convert(input, Ident::to_constant_name),
        TestCasing::ScreamingSnake,
    );
}

#[test]
fn to_crate_name() {
    TEST_CASES.test(str::to_crate_name, TestCasing::Snake);
    TEST_CASES_STR_ONLY.test(str::to_crate_name, TestCasing::Snake);
    #[cfg(feature = "syn")]
    TEST_CASES.test(
        |input| TestIdent::convert(input, Ident::to_crate_name),
        TestCasing::Snake,
    );
}

#[test]
fn to_field_name() {
    TEST_CASES.test(str::to_field_name, TestCasing::Snake);
    TEST_CASES_STR_ONLY.test(str::to_field_name, TestCasing::Snake);
    #[cfg(feature = "syn")]
    TEST_CASES.test(
        |input| TestIdent::convert(input, Ident::to_field_name),
        TestCasing::Snake,
    );
}

#[test]
fn to_function_name() {
    TEST_CASES.test(str::to_function_name, TestCasing::Snake);
    TEST_CASES_STR_ONLY.test(str::to_function_name, TestCasing::Snake);
    #[cfg(feature = "syn")]
    TEST_CASES.test(
        |input| TestIdent::convert(input, Ident::to_function_name),
        TestCasing::Snake,
    );
}

#[test]
fn to_lifetime_name() {
    TEST_CASES.test(str::to_lifetime_name, TestCasing::LifetimeName);
    TEST_CASES_STR_ONLY.test(str::to_lifetime_name, TestCasing::LifetimeName);
    #[cfg(feature = "syn")]
    TEST_CASES.test(
        |input| TestIdent::convert(input, |ident| Ident::to_lifetime_name(ident).to_string()),
        TestCasing::LifetimeName,
    );
}

#[test]
fn to_macro_name() {
    TEST_CASES.test(str::to_macro_name, TestCasing::Snake);
    TEST_CASES_STR_ONLY.test(str::to_macro_name, TestCasing::Snake);
    #[cfg(feature = "syn")]
    TEST_CASES.test(
        |input| TestIdent::convert(input, Ident::to_macro_name),
        TestCasing::Snake,
    );
}

#[test]
fn to_module_name() {
    TEST_CASES.test(str::to_module_name, TestCasing::Snake);
    TEST_CASES_STR_ONLY.test(str::to_module_name, TestCasing::Snake);
    #[cfg(feature = "syn")]
    TEST_CASES.test(
        |input| TestIdent::convert(input, Ident::to_module_name),
        TestCasing::Snake,
    );
}

#[test]
fn to_variable_name() {
    TEST_CASES.test(str::to_variable_name, TestCasing::Snake);
    TEST_CASES_STR_ONLY.test(str::to_variable_name, TestCasing::Snake);
    #[cfg(feature = "syn")]
    TEST_CASES.test(
        |input| TestIdent::convert(input, Ident::to_variable_name),
        TestCasing::Snake,
    );
}

#[test]
fn to_primitive_name() {
    TEST_CASES.test(str::to_primitive_name, TestCasing::Flat);
    TEST_CASES_STR_ONLY.test(str::to_primitive_name, TestCasing::Flat);
    #[cfg(feature = "syn")]
    TEST_CASES.test(
        |input| TestIdent::convert(input, Ident::to_primitive_name),
        TestCasing::Flat,
    );
}

#[test]
fn to_static_name() {
    TEST_CASES.test(str::to_static_name, TestCasing::ScreamingSnake);
    TEST_CASES_STR_ONLY.test(str::to_static_name, TestCasing::ScreamingSnake);
    #[cfg(feature = "syn")]
    TEST_CASES.test(
        |input| TestIdent::convert(input, Ident::to_static_name),
        TestCasing::ScreamingSnake,
    );
}

#[test]
fn to_trait_name() {
    TEST_CASES.test(str::to_trait_name, TestCasing::Pascal);
    TEST_CASES_STR_ONLY.test(str::to_trait_name, TestCasing::Pascal);
    #[cfg(feature = "syn")]
    TEST_CASES.test(
        |input| TestIdent::convert(input, Ident::to_trait_name),
        TestCasing::Pascal,
    );
}

#[test]
fn to_type_name() {
    TEST_CASES.test(str::to_type_name, TestCasing::Pascal);
    TEST_CASES_STR_ONLY.test(str::to_type_name, TestCasing::Pascal);
    #[cfg(feature = "syn")]
    TEST_CASES.test(
        |input| TestIdent::convert(input, Ident::to_type_name),
        TestCasing::Pascal,
    );
}

#[test]
fn to_variant_name() {
    TEST_CASES.test(str::to_variant_name, TestCasing::Pascal);
    TEST_CASES_STR_ONLY.test(str::to_variant_name, TestCasing::Pascal);
    #[cfg(feature = "syn")]
    TEST_CASES.test(
        |input| TestIdent::convert(input, Ident::to_variant_name),
        TestCasing::Pascal,
    );
}
