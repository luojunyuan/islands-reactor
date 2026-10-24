use std::cell::RefCell;

use windows_core::{Interface, Result, implement};

use crate::bindings::*;
use crate::bindings_iuxc as Iuxc;

#[implement(IXamlMetadataProvider)]
struct IslandApplicationOverrides {
    iuxc_provider: RefCell<Option<Iuxc::XamlMetaDataProvider>>,
}

impl IslandApplicationOverrides {
    fn new() -> Self {
        Self {
            iuxc_provider: RefCell::new(None),
        }
    }

    fn iuxc_provider(&self) -> Result<IXamlMetadataProvider> {
        if let Some(provider) = self.iuxc_provider.borrow().as_ref() {
            return provider.cast();
        }

        let provider = Iuxc::XamlMetaDataProvider::new()?;
        *self.iuxc_provider.borrow_mut() = Some(provider.clone());
        provider.cast()
    }
}

#[allow(non_snake_case)]
impl IXamlMetadataProvider_Impl for IslandApplicationOverrides_Impl {
    fn GetXamlType(&self, r#type: &TypeName) -> Result<IXamlType> {
        self.iuxc_provider()?.GetXamlType(r#type)
    }

    fn GetXamlTypeByFullName(&self, full_name: &windows_core::HSTRING) -> Result<IXamlType> {
        let full_name = full_name.to_string_lossy();
        self.iuxc_provider()?.GetXamlTypeByFullName(&full_name)
    }

    fn GetXmlnsDefinitions(&self) -> Result<windows_core::Array<XmlnsDefinition>> {
        self.iuxc_provider()?.GetXmlnsDefinitions()
    }
}

pub(crate) fn create_island_application() -> Result<Application> {
    Application::compose(IslandApplicationOverrides::new())
}
