<p align="center">
    <img src="images/program_icon.png" />
</p>

## About

![Altccents screen](images/altccents_screen.png)

**Altccents** is an utility for Windows that offers an alternative way to type accented characters. The utility is designed for **_qwerty_** keyboard layout and originally supports french accented characters.

There are several programs that perform a similar tasks. But none of them fully suit my specific needs. That project was inspired by [QuickAccent](https://aka.ms/PowerToysOverview_QuickAccent).

## Features

-   Type accented characters without changing the keyboard layout
-   Select accents by using "**Accent control key**"(Num Lock by default)
-   Manage and monitor utility state using tray icon

## How to use

Altcents supports accents for the following characters:

| **Char** | **Key** |
| :------: | :-----: |
|    A     |    A    |
|    E     |    E    |
|    I     |    I    |
|    O     |    O    |
|    U     |    U    |
|    C     |    C    |
|    Y     |    Y    |
|    €     |    '    |

All controls are built around "**Accent control key**"(Num Lock).

To type accent hold down **Num Lock** and press character key. For example, if you want to type _ö_, press and hold **Num Lock** and press **o** 2 times.

![Usage screen](images/usage_screen.png)

To turn on/off altccents there is a shortcut: **Win + Shift + Num Lock**.
You can also manage altccents by interacting with the **tray icon**.

## Installation

1. Download [Altccents](https://github.com/Clovis1444/altccents/releases/latest)
2. Place **altccents.exe** into any directory
3. Run **altccents.exe**, right-click on the application tray icon and select **_Add to startup_**. Alternatively, create a shortcut to **altcents.exe** in `%APPDATA%\Microsoft\Windows\Start Menu\Programs\Startup\`.

## Configuration

**Altccents** may be configured through [setting options](#here-is-the-list-of-all-available-setting-options). To configure application do the following:

1. Copy preferred setting options to your clipboard. Example: copy **_nosound circle_** to disable application sounds and change selection figure to circle.
2. Open **_Altccent context menu_** by right-clicking on the application tray icon and select **Set settings from clipboard** option.

You can also pass any combination of setting options to [**Altccents shortcut**](#installation).

### Here is the list of all available **setting options**:

-   `controlKey<key>` - set **Accent control key**.

    **_key_** must have one of the following values:

    -   _144_ - NumLock
    -   _145_ - ScrollLock
    -   _123_ - F12

    Example: **_controlKey144_**.

-   `timer<ms>` - causes a letter to be printed if there is no input from the user within a **_ms_** milliseconds. **_ms_** is optional, if **_ms_** is not specified timer will be set to 1 second by default, otherwise timer will be set on **_ms_** milliseconds.

    Example: **_timer3000_**.

-   `noTimer` - disable timer.
-   `noSound` - disable application sounds.
-   `sound` - enable application sounds.
-   `disable` - application will start in **disabled** state.
-   `enable` - application will start in **enabled** state.
-   `fontSize<size>` - set popup window font size.

    Example: **_fontSize30_**.

-   `transp<val>` - set popup window transparancy. **_val_** must be in range [0..255], where 0 is fully transparent and 255 is opaque.

    Example: **_transp230_**.

-   `cellSize<size>` - set popup window cell size.

    Example: **_cellSize40_**.

-   `round<factor>` - set popup _cell rounding factor_. Default **_factor_** is _7_. Use **_round1_** for max rounding and enabling **circle selection**.

    Example: **_round4_**.

-   `selectScale<factor>` - set popup _select cell_ scale factor. Default **_factor_** is _0.9_; use **_selectScale1_** to disable scaling.

    Example: **_selectScale0.8_**.

-   `circleSelect` - change popup selection figure to circle instead of rounded rectangle.
-   `rectSelect` - change popup selection figure to rounded rectangle.

> [!NOTE]
> Letter case does not matter in option names.

> [!TIP]
> These parameters are equivalent to the default settings:
>
> ```
> controlKey144 noTimer sound enable transp255 fontSize50 cellSize70 round7 selectScale0.9 rectSelect
> ```
