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

WebUI.navigateToUrl('https://pondokmakankangnaryo.store/pesanan')

WebUI.setText(findTestObject('Object Repository/Page_Restaurant Order/input_Welcome to Our Restaurant_customer-name'), 'Irfan')

WebUI.click(findTestObject('Object Repository/Page_Restaurant Order/button_Start Order'))

WebUI.verifyElementVisible(findTestObject('Object Repository/Page_Restaurant Order/div_Keranjang masih kosong          Silakan_824f3c'))

WebUI.click(findTestObject('Page_Restaurant Order/button_Bakso_add-item-btn w-8 h-8 flex items-center justify-center bg-primary text-white rounded-full'))

WebUI.verifyElementVisible(findTestObject('Object Repository/Page_Restaurant Order/button_Buat Struk'))

WebUI.closeBrowser()

