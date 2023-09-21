# Parse Error in `quick_xml`

When using a version of `quick_xml` ≥ 0.27, an erroneous parse error happens.

The following XML is received from an SAP OData server:

```xml
<ReferentialConstraint>
  <Principal Role="FromRole_Assoc_VH_UnitLength_Products">
    <PropertyRef Name="Msehi"/>
  </Principal>
  <Dependent Role="ToRole_Assoc_VH_UnitLength_Products">
    <PropertyRef Name="DimUnit"/>
  </Dependent>
</ReferentialConstraint>
```

As you can see, the `Name` property is only present on the `<PropertyRef>` element, and this XML parses fine using versions of `quick-xml` up to `0.26`.

However, as soon as I switch to any version from `0.27` or higher, I get the following parse error `Custom("missing field ``Name``")'`.

This is appears to be a bug in `quick-xml`...
