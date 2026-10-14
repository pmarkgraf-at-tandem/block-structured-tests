#[cfg(test)]
mod tests {
    extern crate speculate;
    use speculate::speculate;
    use transient_or_persistent::IsTransient;

    speculate! {
        describe "given an object with zero value" {
            before {
                #[allow(unused_mut)]
                let mut sut = IsTransient::new();
            }
            context "when not incremented" {
                it then_report_0 {
                    assert_eq!(0, sut.report());
                }
            }
            context "when incremented" {
                before {
                    sut.increment();
                }
                it then_report_1 {
                    assert_eq!(1, sut.report());
                }

                context "when incremented again" {
                    before {
                        sut.increment();
                    }
                    it then_report_2 {
                        assert_eq!(2, sut.report());
                    }
                }
            }

            context "when another not incremented" {
                it then_report_0 {
                    assert_eq!(0, sut.report());
                }
            }
        }
    }
}
