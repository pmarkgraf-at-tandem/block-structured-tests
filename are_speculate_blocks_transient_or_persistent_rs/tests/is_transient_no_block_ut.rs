#[cfg(test)]
mod tests {
    extern crate speculate;
    use speculate::speculate;
    use transient_or_persistent::IsTransient;

    speculate! {
        describe "given an object with zero value a" {
            context "when not incremented" {
                it then_report_0 {
                    let sut = IsTransient::new();
                    assert_eq!(0, sut.report());
                }
            }
        }
        describe "given an object with zero value b" {
            context "when incremented" {
                it then_report_1 {
                    let mut sut = IsTransient::new();
                    sut.increment();
                    assert_eq!(1, sut.report());
                }
            }
        }

        describe "given an object with zero value c" {
            context "when incremented" {
                context "when incremented again" {
                    it then_report_2 {
                        let mut sut = IsTransient::new();
                        sut.increment();
                        sut.increment();
                        assert_eq!(2, sut.report());
                    }
                }
            }
        }

        describe "given an object with zero value d" {
            context "when another not incremented" {
                it then_report_0 {
                    let sut = IsTransient::new();
                    assert_eq!(0, sut.report());
                }
            }
        }
    }
}
