#[cfg(test)]
mod tests {
    use behave::prelude::*;
    use transient_or_persistent::IsTransient;

    behave! {
        "given an object with zero value a" {
            "when not incremented" {
                "then reports zero" {
                    let sut = IsTransient::new();
                    assert_eq!(0, sut.report());
                }
            }
        }
        "given an object with zero value b" {
            "when incremented" {
                "then reports one" {
                    let mut sut = IsTransient::new();
                    sut.increment();
                    assert_eq!(1, sut.report());
                }
            }
        }

        "given an object with zero value c" {
            "when incremented" {
                "when incremented again" {
                    "then reports two" {
                        let mut sut = IsTransient::new();
                        sut.increment();
                        sut.increment();
                        assert_eq!(2, sut.report());
                    }
                }
            }
        }

        "given an object with zero value d" {
            "when another not incremented" {
                "then reports zero" {
                    let sut = IsTransient::new();
                    assert_eq!(0, sut.report());
                }
            }
        }
    }
}
