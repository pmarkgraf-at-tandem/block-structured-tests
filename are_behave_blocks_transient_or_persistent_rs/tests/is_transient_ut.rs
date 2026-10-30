#[cfg(test)]
mod tests {
    use behave::prelude::*;
    use transient_or_persistent::IsTransient;

    behave! {
        "given an object with zero value" {
            setup {
                #[allow(unused_mut)]
                let mut sut = IsTransient::new();
            }
            "when not incremented" {
                "then reports zero" {
                    assert_eq!(0, sut.report());
                }
            }
            "when incremented" {
                setup {
                    sut.increment();
                }
                "then reports one" {
                    assert_eq!(1, sut.report());
                }

                "when incremented again" {
                    setup {
                        sut.increment();
                    }
                    "then reports two" {
                        assert_eq!(2, sut.report());
                    }
                }
            }

            "when another not incremented" {
                "then reports zero" {
                    assert_eq!(0, sut.report());
                }
            }
        }
    }
}
