# Flags to Futures | CTFd-branded homepage

1. In CTFd, open **Admin Panel → Pages** and edit/create the home page at route `index`.
2. Select **HTML** mode and paste the contents of `homepage.html`. Do not paste a full `<html>` document.
3. Open **Admin Panel → Config → Theme** and paste `theme-header.html` into the theme header field.
4. Save and refresh the participant homepage. Test in a private browser window.

The CSS is scoped to `.zth-home` so it should not affect the CTFd admin panel. The internal `zth-` class prefix is retained to preserve existing CSS compatibility.

## Copy buttons on the GitHub Pages website
Visit `https://kristinechristensen.github.io/ctf/#customize` and use **Copy homepage HTML** and **Copy theme CSS**. The buttons load the actual files in this folder and copy their complete contents. If you change either file in GitHub, the button automatically copies the updated version.

Do not paste the CSS into the homepage editor; it belongs in **Config → Theme → Theme Header**. The theme-header file includes its surrounding `<style>` element.
