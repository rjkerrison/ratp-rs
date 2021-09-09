# Thing

Generated with savon, but errors meant needing to fix things manually.

## Fixes made to Savon generated code

1. In many cases, `,)` appeared at the end of a `Some` enum creation. In every case, this needed to be switched to `),`. Savon should handle this correctly, with the `,` as a separator between properties appearing outside the `Some()`.
2. In the WSDL file I am using, there are attributes called `type` on some of the objects. Savon generated for these a property on the corresponding struct called `type` verbatim. As this is a keyword, the code was invalid. Savon should have a policy in place to map these keywords to something valid. My fix was to use `r_type` as the name.
