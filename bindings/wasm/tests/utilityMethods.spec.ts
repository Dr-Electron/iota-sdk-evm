import { Utils } from '../node/lib';

describe('Utils methods', () => {
    it('generates and validates mnemonic', async () => {
        const mnemonic = Utils.generateMnemonic();

        // A mnemonic has 24 words
        expect(mnemonic.split(' ').length).toEqual(24);

        Utils.verifyMnemonic(mnemonic);
        try {
            Utils.verifyMnemonic('invalid mnemonic '.repeat(12));
            throw 'should error';
        } catch (e) {
            expect(e.message).toContain('NoSuchWord');
        }
    });
});
