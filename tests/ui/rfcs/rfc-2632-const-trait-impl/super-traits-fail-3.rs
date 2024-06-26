#![feature(const_trait_impl, effects)] //~ WARN the feature `effects` is incomplete

//@ revisions: yy yn ny nn
//@[yy] check-pass

#[cfg_attr(any(yy, yn), const_trait)]
trait Foo {
    fn a(&self);
}

#[cfg_attr(any(yy, ny), const_trait)]
trait Bar: ~const Foo {}
//[ny,nn]~^ ERROR: `~const` can only be applied to `#[const_trait]`
//[ny,nn]~| ERROR: `~const` can only be applied to `#[const_trait]`
//[ny,nn]~| ERROR: `~const` can only be applied to `#[const_trait]`
//[yn,nn]~^^^^ ERROR: `~const` is not allowed here

const fn foo<T: ~const Bar>(x: &T) {
    //[yn,nn]~^ ERROR: `~const` can only be applied to `#[const_trait]`
    x.a();
    //[yn]~^ ERROR: mismatched types
}

fn main() {}
