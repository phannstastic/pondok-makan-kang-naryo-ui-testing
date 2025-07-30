import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('https://pondokmakankangnaryo.store/admin/login')

WebUI.setText(findTestObject('Object Repository/Page_Login - Laravel/input__data.email_1'), 'adminpondokmakan@gmail.com')

WebUI.setEncryptedText(findTestObject('Object Repository/Page_Login - Laravel/input__data.password'), 'hUKwJTbofgPU9eVlw/CnDQ==')

WebUI.click(findTestObject('Object Repository/Page_Login - Laravel/button_Sign in'))

WebUI.click(findTestObject('Object Repository/Page_Dashboard - Laravel/span_Menu Items'))

WebUI.click(findTestObject('Object Repository/Page_Menu Items - Laravel/span_New menu item'))

WebUI.setText(findTestObject('Object Repository/Page_Create Menu Item - Laravel/input__data.name'), 'Mie Ayam Ceker')

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_Create Menu Item - Laravel/select_Select an option                    _63aa95'), 
    'makanan', true)

WebUI.setText(findTestObject('Object Repository/Page_Create Menu Item - Laravel/textarea_Deskripsi_data.description'), 'Mie Ayam dengan tambahan Ceker')

WebUI.setText(findTestObject('Object Repository/Page_Create Menu Item - Laravel/input_Rp_data.price'), '15000')

WebUI.uploadFile(findTestObject('Page_Create Menu Item - Laravel/div_Drag  Drop your files or  Browse'), image_menu)

WebUI.delay(10)

WebUI.click(findTestObject('Object Repository/Page_Create Menu Item - Laravel/button_Create'))

WebUI.verifyElementVisible(findTestObject('Object Repository/Page_Menu Items - Laravel/h3_Created'))

WebUI.closeBrowser()

