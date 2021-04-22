//I couldn't get ansi_up to play nicely with webpack, so this is the solution

/**
 * TypeScript bindings for the ansi_up NPM package
 */
declare class AnsiUp {
    /**
     * This transforms ANSI terminal escape codes/sequences into SPAN tags that wrap and style the content.
     * 
     * This method only interprets ANSI SGR (Select Graphic Rendition) codes or escaped URL codes.
     * For example, cursor movement codes are ignored and hidden from output.
     *
     * This method also safely escapes any unsafe HTML characters.
     *
     * The default style uses colors that are very close to the prescribed standard. 
     * The standard assumes that the text will have a black background. 
     * These colors are set as inline styles on the SPAN tags. 
     * Another option is to set the 'use_classes' property to true'. 
     * This will instead set classes on the spans so the colors can be set via CSS. 
     * The class names used are of the format ansi-*-fg/bg and 
     * ansi-bright-*-fg/bg where * is the colour name, i.e black/red/green/yellow/blue/magenta/cyan/white. 
     * See the examples directory for a complete CSS theme for these classes.
     *
     * @param input The input to transform
     */
    ansi_to_html(input: string): string;
}

export default AnsiUp;