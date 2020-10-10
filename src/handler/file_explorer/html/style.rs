pub const STYLE: &str = r##"
body {
  background-color: #EFEFEF;
  color: #171B1F;
  font-family: sans-serif;
  margin: 0;
  padding: 0;
}

.file-icon {
  background-image: url("data:image/svg+xml", "%3Csvg height='20px' width='30px'  fill="#437CB0" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" x="0px" y="0px"%3E%3Cg data-name="22"%3E%3Cpath d="M21,7H12.72L12,4.68A1,1,0,0,0,11,4H3A1,1,0,0,0,2,5V19a1,1,0,0,0,1,1H21a1,1,0,0,0,1-1V8A1,1,0,0,0,21,7Z"%3E%3C/path%3E%3C/g%3E%3C/svg%3E");
  display: inline-block;
  height: 20px;
  width: 30px;
}

.primary {
  color: #7DBDA3;
}

.secondary {
  color: #437CB0;
}

.danger {
  color: #DD6272;
}

.warning {
  color: #E6A04C;
}

#current-directory {
  background-color: #F7F7F7;
  box-sizing: border-box;
  color: #89909A;
  padding: 1rem .5rem;
}

#current-directory #container {
  /* display: grid; */
  /* grid-template-columns: minmax(200px, 30%) minmax(400px, 70%); */
  margin: 0 auto;
  width: 95%;
}

#current-directory #container #dirname {
  /* grid-column: 1 / 1; */
}

#current-directory #container #dirname h2 {
  margin: 0;
  margin-bottom: 1rem;
  padding: 0;
  text-align: left;
}

.code {
  background-color: #EFEFEF;
  color: #DD6272;
  border-radius: .25rem;
  margin: 0;
  padding: .3rem .6rem;
  text-align: left;
  letter-spacing: .1rem;
}

#current-directory #container #dirname code {
  margin-bottom: 1rem;
}

#current-directory #container #toolbar {
  /* grid-column: 2 / 2; */
}

#file-table {
  border-collapse: collapse;
  margin: 0 auto;
  width: 95%;
}

#file-table thead {
  text-align: left;
}

#file-table thead th {
  box-sizing: border-box;
  color: #7c7c7c;
  font-weight: 300;
  padding: 1rem;
}

#file-table tbody {
  background-color: #ffffff;
}

#file-table tbody tr td {
  box-sizing: border-box;
  padding: 1rem;
}

#file-table tbody tr td a {
  color: #437CB0;
  cursor: pointer;
  text-decoration: underline;
}

#file-table tbody tr:hover {
  background-color: #f8f8f8;
}

#fs-footer {
  box-sizing: border-box;
  margin: 0 auto;
  padding: 1rem;
  text-align: center;
  width: 95%;
}

#fs-footer small {
  color: #89909A;
}

#fs-footer small {
  color: #89909A;
}

#icon-th {
  width: 35px;
}
"##;
