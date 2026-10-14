const IsTransient = require('../src/is_transient');

describe('given… an object with zero value', function () {
    describe('when… not incremented', function () {
        it('then… report 0', function () {
            let sut = new IsTransient;
            expect(sut.report()).toBe(0);
        });
    });
});

describe('given… an object with zero value', function () {
    describe('when… incremented', function () {
        it('then… report 1', function () {
            let sut = new IsTransient;
            sut.increment();
            expect(sut.report()).toBe(1);
        });
    });
});

describe('given… an object with zero value', function () {
    describe('when… incremented', function () {
        describe('when… incremented again', function () {
            it('then… report 2', function () {
                let sut = new IsTransient;
                sut.increment();
                sut.increment();
                expect(sut.report()).toBe(2);
            });
        });
    });
});

describe('given… an object with zero value', function () {
    describe('when… another not incremented', function () {
        it('then… report 0', function () {
            let sut = new IsTransient;
            expect(sut.report()).toBe(0);
        });
    });
});
