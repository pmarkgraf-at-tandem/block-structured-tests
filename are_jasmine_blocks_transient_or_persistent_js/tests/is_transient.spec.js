const IsTransient = require('../src/is_transient');

describe('given… an object with zero value', function () {
    let sut;
    beforeEach(() => {
        sut = new IsTransient;
    });

    describe('when… not incremented', function () {
        it('then… report 0', function () {
            expect(sut.report()).toBe(0);
        });
    });

    describe('when… incremented', function () {
        beforeEach(() => {
            sut.increment();
        });
        it('then… report 1', function () {
            expect(sut.report()).toBe(1);
        });

        describe('when… incremented again', function () {
            beforeEach(() => {
                sut.increment();
            });
            it('then… report 2', function () {
                expect(sut.report()).toBe(2);
            });
        });
    });

    describe('when… another not incremented', function () {
        it('then… report 0', function () {
            expect(sut.report()).toBe(0);
        });
    });
});
