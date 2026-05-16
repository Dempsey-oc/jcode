# jcode-azure-auth

_Platform/support utility_

Azure credential helpers built on `azure_identity`. Used by the Azure OpenAI
provider and any other crate that needs an AAD bearer token.

## Layout

Source root: [`src/`](src/)

## Dependencies

anyhow, azure_core, azure_identity

See [`docs/CRATE_OWNERSHIP_BOUNDARIES.md`](../../docs/CRATE_OWNERSHIP_BOUNDARIES.md) for the rules governing what does and does not belong here.
