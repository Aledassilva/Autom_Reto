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

WebUI.navigateToUrl('https://opencart.abstracta.us/')

WebUI.maximizeWindow()

WebUI.waitForPageLoad(30)

WebUI.scrollToPosition(0, 140)

WebUI.click(findTestObject('Object Repository/CP_E2E/Page_Your Store/span_Add to Cart'))

WebUI.delay(variable_tiempo)

WebUI.click(findTestObject('Object Repository/CP_E2E/Page_Your Store/span_Add to Cart_1'))

WebUI.delay(variable_tiempo)

WebUI.click(findTestObject('Object Repository/CP_E2E/Page_Your Store/button_2 item(s) - 725.20'))

WebUI.delay(variable_tiempo)

WebUI.click(findTestObject('Object Repository/CP_E2E/Page_Your Store/strong_View Cart'))

WebUI.delay(variable_tiempo)

WebUI.click(findTestObject('Object Repository/CP_E2E/Page_Shopping Cart/a_Checkout'))

WebUI.delay(variable_tiempo)

WebUI.click(findTestObject('Object Repository/CP_E2E/Page_Checkout/input_Guest Checkout_account'))

WebUI.delay(variable_tiempo)

WebUI.click(findTestObject('Object Repository/CP_E2E/Page_Checkout/input_Guest Checkout_button-account'))

WebUI.delay(variable_tiempo)

WebUI.setText(findTestObject('Object Repository/CP_E2E/Page_Checkout/input_First Name_firstname'), 'Alejandro')

WebUI.setText(findTestObject('Object Repository/CP_E2E/Page_Checkout/input_Last Name_lastname'), 'Tinjaca Henao')

WebUI.setText(findTestObject('Object Repository/CP_E2E/Page_Checkout/input_E-Mail_email_1'), 'sofka@sofka.com.co')

WebUI.setText(findTestObject('Object Repository/CP_E2E/Page_Checkout/input_Telephone_telephone'), '3004007495')

WebUI.setText(findTestObject('Object Repository/CP_E2E/Page_Checkout/input_Company_company'), 'sofka')

WebUI.setText(findTestObject('Object Repository/CP_E2E/Page_Checkout/input_Address 1_address_1'), 'calle 84#985 bis')

WebUI.setText(findTestObject('Object Repository/CP_E2E/Page_Checkout/input_City_city'), 'bogota')

WebUI.setText(findTestObject('Object Repository/CP_E2E/Page_Checkout/input_Post Code_postcode'), '0000000')

WebUI.selectOptionByValue(findTestObject('Object Repository/CP_E2E/Page_Checkout/select_--- Please Select ---               _6a058e'), 
    '47', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/CP_E2E/Page_Checkout/select_--- Please Select --- AmazonasAntioq_dbd715'), 
    '724', true)

WebUI.delay(variable_tiempo)

WebUI.click(findTestObject('Object Repository/CP_E2E/Page_Checkout/input_My delivery and billing addresses are_99b3c6'))

WebUI.setText(findTestObject('Object Repository/CP_E2E/Page_Checkout/textarea_Add Comments About Your Order_comment'), 'prueba de escritura 1')

WebUI.click(findTestObject('Object Repository/CP_E2E/Page_Checkout/input_Add Comments About Your Order_button-_543ebe'))

WebUI.click(findTestObject('Object Repository/CP_E2E/Page_Checkout/input_Terms  Conditions_agree'))

WebUI.click(findTestObject('Object Repository/CP_E2E/Page_Checkout/input_Terms  Conditions_button-payment-method'))

WebUI.click(findTestObject('Object Repository/CP_E2E/Page_Checkout/input_Please transfer the total amount to t_a6d3bd'))

WebUI.delay(variable_tiempo)

WebUI.verifyElementText(findTestObject('Object Repository/CP_E2E/Page_Your order has been placed/h1_Your order has been placed'), 
    'Your order has been placed!')

WebUI.delay(variable_tiempo)

WebUI.closeBrowser()

